use std::fmt;

use prelude::*;
use ::db::{UserFilter};
use super::tokens::{Token, TokenCreate, TokenKind};

#[derive(Fail, Debug)]
#[fail(display = "user_not_found)")]
pub struct UserNotFoundError;

#[derive(Fail, Debug)]
#[fail(display = "invalid_password)")]
pub struct InvalidPasswordError;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub enum UserRole {
    Admin,
    Reviewer,
    Translator,
    Viewer,
}

impl UserRole {
    pub fn parse(val: &str) -> Self {
        use self::UserRole::*;
        match val {
            "admin" => Admin,
            "reviewer" => Reviewer,
            "translator" => Translator,
            "viewer" => Viewer,
            _ => {
                panic!("Unknown role: {}", val);
            }
        }
    }
}

impl fmt::Display for UserRole {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::UserRole::*;
        let s = match self {
            Admin => "admin",
            Reviewer => "reviewer",
            Translator => "translator",
            Viewer => "viewer",
        };
        write!(f, "{}", s)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub id: Uuid,
    pub role: UserRole,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: Option<DateTime>,
}

pub fn root_user() -> User {
    User {
        id: "00000000-0000-0000-0000-000000000001".parse().unwrap(),
        role: UserRole::Admin,
        email: "admin@translator.com".into(),
        username: "admin".into(),
        password_hash: "pw".into(),
        created_at: now(),
        updated_at: now(),
        deleted_at: None,
    }
}

impl From<db::types::User> for User {
    fn from(u: db::types::User) -> User {
        User {
            id: u.id.parse().unwrap(),
            role: UserRole::parse(&u.role),
            email: u.email,
            username: u.username,
            password_hash: u.password_hash,
            created_at: utc_from_naive(u.created_at),
            updated_at: utc_from_naive(u.updated_at),
            deleted_at: u.deleted_at.map(utc_from_naive),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserCreate {
    pub role: String,
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserUpdate {
    pub id: Uuid,
    pub role: Option<UserRole>,
    pub email: Option<String>,
    pub username: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserUpdatePassword {
    pub id: Uuid,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserLogin {
    pub user: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserLoginResult {
    pub user: User,
    pub token: Token,
}

pub struct Users {
    app: App,
}

impl Users {
    pub fn new(app: App) -> Self {
        Users { app }
    }

    fn hash_password(pw: &str) -> Result<String, ::bcrypt::BcryptError> {
        use bcrypt::{hash, verify, DEFAULT_COST};
        hash(pw, DEFAULT_COST)
    }

    fn verify_password(pw: &str, user: &User) -> Result<bool, ::bcrypt::BcryptError> {
        use bcrypt::verify;
        verify(pw, &user.password_hash)
    }

    pub fn find(&self, id: &Uuid) -> Result<User, Error> {
        let u = self.app.db()?.user(&id.to_string())?;
        Ok(u.into())
    }

    pub fn filter(&self, filter: &UserFilter) -> Result<Vec<User>, Error> {
        let users = self.app.db()?.users(filter)?
            .into_iter()
            .map(|x| -> User { x.into() })
            .collect();
        Ok(users)
    }

    pub fn create(&self, data: UserCreate) -> Result<User, Error> {
        let create = db::types::UserCreate {
            id: uuid().to_string(),
            role: data.role.to_string(),
            email: data.email,
            username: data.username,
            password_hash: Self::hash_password(&data.password)?,
        };
        Ok(self.app.db()?.user_create(&create)?.into())
    }

    pub fn update(&self, data: UserUpdate) -> Result<User, Error> {
        let user = self.find(&data.id)?;
        let update = db::types::UserUpdate {
            id: data.id.to_string(),
            role: data.role.unwrap_or(user.role).to_string(),
            email: data.email.unwrap_or(user.email),
            username: data.username.unwrap_or(user.username).to_string(),
            updated_at: now().naive_utc(),
        };
        let u = self.app.db()?.user_update(&update)?;
        Ok(u.into())
    }

    pub fn update_password(&self, data: UserUpdatePassword) -> Result<User, Error> {
        let update = db::types::UserUpdatePassword {
            id: data.id.to_string(),
            password_hash: Self::hash_password(&data.password)?,
            updated_at: now().naive_utc(),
        };
        let u = self.app.db()?.user_update_password(&update)?;
        Ok(u.into())
    }

    pub fn ensure_admin_user(&self) -> Result<User, Error> {
        let users = self.filter(&UserFilter{
            username: Some("admin".into()),
            email: None,
        })?;
        if users.len() > 0 {
            Ok(users[0].clone())
        } else {

            let u = UserCreate{
                role: UserRole::Admin.to_string(),
                email: "admin@translator.io".into(),
                username: "admin".into(),
                password: self.app.config.admin_password.clone(),
            };
            self.create(u)
        }
    }

    pub fn login(&self, data: UserLogin) -> Result<UserLoginResult, Error> {
        let db = self.app.db()?;

        // First, find the user.
        let mut users = self.filter(&::db::UserFilter{
            username: Some(data.user.clone()),
            email: None,
        })?;
        if users.len() != 1 {
            users = self.filter(&::db::UserFilter{
                username: None,
                email: Some(data.user.clone()),
            })?;
        }

        if users.len() != 1 {
            return Err(format_err!("user_not_found"));
        }
        let user = &users[0];

        // Validate password.
        if Self::verify_password(&data.password, user)? == false {
            return Err(format_err!("invalid_password"));
        }

        // Password OK, generate token.
        let token_create = TokenCreate{
            kind: TokenKind::Auth,
            token: uuid().to_string(),
            valid_until: None,
        };
        let token = self.app.tokens().create(token_create)?;

        let mut user = user.clone();
        user.password_hash = "".into();
        Ok(UserLoginResult{
            user,
            token,
        })
    }
}

use chrono::Utc;

use ::error::*;
use super::{ApiToken, TokenKind};

table!(
  users(username) {
    id -> Text,
    username -> Text,
    role -> Text,
    password_hash -> Text,
    created_at -> BigInt,
  }
);

#[derive(Debug, Clone, Copy)]
pub enum Role {
    User,
    Admin,
}

impl Role {
    pub fn to_str(&self) -> &'static str {
        match *self {
            Role::User => "user",
            Role::Admin => "admin",
        }
    }

    pub fn from_str<S: AsRef<str>>(raw: S) -> Result<Role> {
        let role = match raw.as_ref() {
            "user" => Role::User,
            "admin" => Role::Admin,
            _ => {
                return Err(ErrorKind::InvalidRole.into());
            },
        };
        Ok(role)
    }
}

#[derive(Insertable, Queryable, AsChangeset,
         Serialize, Deserialize, Debug, Clone)]
#[table_name="users"]
pub struct User {
    pub id: String,
    pub username: String,
    pub role: String,
    pub password_hash: String,
    pub created_at: i64,
}

impl User {
    pub fn hash_password<S: AsRef<str>>(pw: S) -> String {
        use ::ring_pwhash::scrypt::{scrypt_simple, ScryptParams};
        let params = ScryptParams::new(4, 2, 1);
        scrypt_simple(pw.as_ref(), &params).unwrap()
    }

    pub fn set_password<S: AsRef<str>>(&mut self, pw: S) {
        self.password_hash = Self::hash_password(pw);
    }

    pub fn new(username: String, role: Role, password: String) -> Self {
        User {
            id: ::uuid::Uuid::new_v4().to_string(),
            username,
            role: role.to_str().into(),
            password_hash: Self::hash_password(password),
            created_at: Utc::now().timestamp(),
        }
    }

  pub fn verify_password<S: AsRef<str>>(&self, pw: S) -> bool {
    use ::ring_pwhash::scrypt::{scrypt_check};
    scrypt_check(pw.as_ref(), &self.password_hash).unwrap_or(false)
  }

    pub fn build_session_token(&self) -> Result<ApiToken> {
        use simple_jwt::{encode, Claim, Algorithm};

        let mut claim = Claim::default();
        claim.set_iss("translator");
        claim.set_iat(Utc::now().timestamp() as u64);
        claim.set_payload_field("username", &self.username);

        let jwt = encode(&claim, "secret", Algorithm::HS256)
            .chain_err(|| "Could not create jwt token")?;

        Ok(ApiToken{
            token: jwt,
            kind: TokenKind::Session.to_str().to_string(),
            created_at: Utc::now().timestamp(),
            expires_at: None,
            created_by: Some(self.id.clone()),
        })
    }
}

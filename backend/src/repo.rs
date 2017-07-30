use chrono::{Utc};

use ::commands::{Ctx};
use ::error::*;
use ::db::{Db, Connection};
use ::app::App;
use ::db::schema::*;

pub struct Repo {
    app: App,
    db: Option<Db>,
}

impl Repo {
    pub fn new(app: App) -> Self {
        Repo {
            app,
            db: None,
        }
    }

    pub fn db(&mut self) -> Result<&Db> {
        if self.db.is_none() {
            self.db = Some(self.app.db()?);
        }
        Ok(self.db.as_ref().unwrap())
    }

    pub fn ensure_admin_user(&mut self) -> Result<()> {
        let db = self.db()?;
        // Ensure admin user exists.
        let admin = db.find_user("admin")?;
        if admin.is_none() {
            eprintln!("Creating admin user...");
            db.create_user("admin", Role::Admin, "admin")?;
        }
        Ok(())
    }

    pub fn login<S: AsRef<str>>(&mut self, username: S, password: S) -> Result<ApiToken> {
        let admin_pw = self.app.config().admin_password.clone();
        let db = self.db()?;

        let username = username.as_ref();
        let password = password.as_ref();

        let mut user = match db.find_user(username)? {
            Some(u) => u,
            None => {
                return Err(ErrorKind::UnknownUser.into());
            },
        };

        if user.username == "admin" && admin_pw == Some(password.to_string()) {
            // Superuser detected.
        } else {
            if !user.verify_password(password) {
                return Err(ErrorKind::InvalidPassword.into());
            }
        }

        let token = user.build_session_token()?;
        db.create_api_token(token)
    }

    pub fn languages(&mut self, user: Option<&User>) -> Result<Vec<Language>> {
        self.db()?.languages()
    }

    pub fn language<S: AsRef<str>>(&mut self, id: S, user: Option<&User>)
        -> Result<Option<Language>>
    {
        self.db()?.language(id)
    }

    pub fn create_language(&mut self, lang: NewLanguage, user: Option<&User>) -> Result<Language> {
        let lang = Language {
            id: lang.id,
            name: lang.name,
            parent_id: lang.parent_id,
            created_by: user.map(|u| u.username.clone()),
            created_at: Utc::now().timestamp(),
        };
        eprintln!("Creating lang: {:?}", lang);
        self.db()?.create_language(lang)
    }

    pub fn delete_language<S: AsRef<str>>(&mut self, language: S, user: Option<&User>) -> Result<()> {
        self.db()?.delete_language(language.as_ref())?;
        Ok(())
    }

    pub fn keys(&mut self, user: Option<&User>) -> Result<Vec<Key>> {
        self.db()?.keys()
    }

    pub fn key<S: AsRef<str>>(&mut self, id: S, user: Option<&User>)
        -> Result<Option<Key>>
    {
        self.db()?.key(id)
    }

    pub fn create_key(&mut self, key: NewKey, user: Option<&User>) -> Result<Key> {
        let key = Key{
            key: key.key,
            description: key.description,
            created_at: Utc::now().timestamp(),
            created_by: user.map(|u| u.username.clone()),
        };
        self.db()?.create_key(key)
    }

    pub fn delete_key<S: AsRef<str>>(&mut self, key: S, user: Option<&User>) -> Result<()> {
        self.db()?.delete_key(key.as_ref())?;
        Ok(())
    }

    pub fn translations<S: AsRef<str>>(&mut self, key: S, user: Option<&User>)
        -> Result<Vec<Translation>>
    {
        self.db()?.translations(key)
    }

    pub fn translate(&mut self, translation: NewTranslation, user: Option<&User>)
        -> Result<Translation>
    {
        // Try to find old translation.
        let t = self.db()?.find_translation(&translation.key, &translation.language)?;


        if let Some(mut t) = t {
            self.db()?.update_translation(
                translation.language.as_ref(),
                translation.key.as_ref(),
                translation.value.as_ref())?;
            t.value = translation.value;
            Ok(t)
        } else {
            let now = Utc::now().timestamp();
            let translation = Translation {
                language: translation.language,
                key: translation.key,
                value: translation.value,
                created_at: now,
                updated_at: now,
                created_by: user.map(|u| u.username.clone()),
            };
            self.db()?.create_translation(translation)
        }
    }

    pub fn update_translation(&mut self, translation: NewTranslation, user: Option<&User>)
        -> Result<Translation>
    {
        let now = Utc::now().timestamp();



        let now = Utc::now().timestamp();
        let translation = Translation {
            language: translation.language,
            key: translation.key,
            value: translation.value,
            created_at: now,
            updated_at: now,
            created_by: None,
        };
        Ok(translation)
    }

    pub fn delete_translation<S: AsRef<str>>(&mut self, language: S, key: S, user: Option<&User>)
        -> Result<()>
    {
        self.db()?.delete_translation(language.as_ref(), key.as_ref())?;
        Ok(())
    }


    pub fn users(&mut self, user: Option<&User>) -> Result<Vec<User>> {
        self.db()?.users()
    }
}
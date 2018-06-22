mod pool;
mod schema;
pub mod types;

pub use self::pool::{build_pool, Pool};

use self::{schema::*, types::*};
use diesel::{self, prelude::*, result::Error};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserFilter {
    pub username: Option<String>,
    pub email: Option<String>,
}

pub struct Db {
    con: pool::PoolConnection,
}

impl Db {
    pub fn new(con: pool::PoolConnection) -> Self {
        Db { con }
    }

    pub fn user(&self, id: &str) -> Result<User, Error> {
        users::table.find(id).get_result(&self.con)
    }

    pub fn users(&self, filter: &UserFilter) -> Result<Vec<User>, Error> {
        use self::users::{table, columns as c};
        let mut q = users::table.into_boxed();
        if let Some(name) = filter.username.as_ref() {
            q = q.filter(c::username.eq(name));
        }
        if let Some(email) = filter.email.as_ref() {
            q = q.filter(c::email.eq(email));
        }
        q.load(&self.con)
    }

    pub fn user_create(&self, user: &UserCreate) -> Result<User, Error> {
        user.insert_into(users::table).execute(&self.con)?;
        self.user(&user.id)
    }

    pub fn user_update(&self, user: &UserUpdate) -> Result<User, Error> {
        diesel::update(users::table).set(user).execute(&self.con)?;
        self.user(&user.id)
    }

    pub fn user_update_password(&self, user: &UserUpdatePassword) -> Result<User, Error> {
        diesel::update(users::table).set(user).execute(&self.con)?;
        self.user(&user.id)
    }

    pub fn token(&self, id: &str) -> Result<Token, Error> {
        tokens::table.find(id).get_result(&self.con)
    }

    pub fn token_create(&self, token: &TokenCreate) -> Result<Token, Error> {
        token.insert_into(tokens::table).execute(&self.con)?;
        self.token(&token.id)
    }

    pub fn language(&self, id: &str) -> Result<Language, Error> {
        languages::table.find(id).get_result(&self.con)
    }

    pub fn language_create(&self, language: &LanguageCreate) -> Result<Language, Error> {
        language.insert_into(languages::table).execute(&self.con)?;
        self.language(&language.id)
    }

    pub fn language_update(&self, language: &LanguageUpdate) -> Result<Language, Error> {
        diesel::update(languages::table)
            .set(language)
            .execute(&self.con)?;
        self.language(&language.id)
    }

    pub fn key(&self, id: &str) -> Result<Key, Error> {
        keys::table.find(id).get_result(&self.con)
    }

    pub fn key_create(&self, key: &KeyCreate) -> Result<Key, Error> {
        key.insert_into(keys::table).execute(&self.con)?;
        self.key(&key.id)
    }

    pub fn key_update(&self, key: &KeyUpdate) -> Result<Key, Error> {
        diesel::update(keys::table).set(key).execute(&self.con)?;
        self.key(&key.id)
    }

    pub fn translation(&self, id: &str) -> Result<Translation, Error> {
        translations::table.find(id).get_result(&self.con)
    }

    pub fn translation_for_key(
        &self,
        key_id: &str,
        lang_id: &str,
    ) -> Result<Option<Translation>, Error> {
        use self::translations::{columns as c, table};
        table
            .filter(c::key_id.eq(key_id))
            .filter(c::language_id.eq(lang_id))
            .order(c::version.desc())
            .first(&self.con)
            .optional()
    }

    pub fn translation_create(
        &self,
        translation: &TranslationCreate,
    ) -> Result<Translation, Error> {
        translation
            .insert_into(translations::table)
            .execute(&self.con)?;
        self.translation(&translation.id)
    }

    pub fn translation_delete(
        &self,
        translation: &TranslationDelete,
    ) -> Result<Translation, Error> {
        diesel::update(translations::table)
            .set(translation)
            .execute(&self.con)?;
        self.translation(&translation.id)
    }

    pub fn translation_request(&self, id: &str) -> Result<TranslationRequest, Error> {
        translation_requests::table.find(id).get_result(&self.con)
    }

    pub fn translation_request_create(
        &self,
        translation_request: &TranslationRequestCreate,
    ) -> Result<TranslationRequest, Error> {
        translation_request
            .insert_into(translation_requests::table)
            .execute(&self.con)?;
        self.translation_request(&translation_request.id)
    }

    pub fn translation_request_update(
        &self,
        translation_request: &TranslationRequestUpdate,
    ) -> Result<TranslationRequest, Error> {
        diesel::update(translation_requests::table)
            .set(translation_request)
            .execute(&self.con)?;
        self.translation_request(&translation_request.id)
    }
}

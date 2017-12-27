use std::time::Duration;

use diesel;
use diesel::prelude::*;
use chrono::{Utc};

use r2d2::{self, PooledConnection};
use diesel::sqlite::SqliteConnection;
use r2d2_diesel::ConnectionManager;

embed_migrations!("./migrations");

use ::error::*;
use super::schema::*;

pub type Connection = SqliteConnection;
pub type PoolConnection = PooledConnection<ConnectionManager<SqliteConnection>>;
pub type Pool = r2d2::Pool<ConnectionManager<Connection>>;

#[derive(Debug)]
struct ConnectionCustomizer;

impl r2d2::CustomizeConnection<Connection, ::r2d2_diesel::Error>
     for ConnectionCustomizer {
    fn on_acquire(&self, conn: &mut Connection) -> ::std::result::Result<(), ::r2d2_diesel::Error> {
      use diesel::connection::SimpleConnection;
      conn.batch_execute(" PRAGMA foreign_keys = ON;").unwrap();
      Ok(())
    }

    fn on_release(&self, _: Connection) {}
}

pub fn build_pool(data_path: &str) -> Result<Pool> {
    let path = ::std::path::PathBuf::from(data_path);
    let db_path = path.join("db.sqlite");
    let db_path = db_path.to_str().unwrap().to_string();

    let manager = ConnectionManager::<Connection>::new(db_path);

    let pool = r2d2::Pool::builder()
        .max_size(1)
        .max_lifetime(Some(Duration::new(10, 0)))
        .idle_timeout(Some(Duration::new(10, 0)))
        .connection_timeout(Duration::new(5, 0))
        .connection_customizer(Box::new(ConnectionCustomizer))
        .build(manager)
        .chain_err(|| "Could not initialize database pool")?;

    {
        let con = &*pool.get()?;
        // Run migrations.
        embedded_migrations::run_with_output(
            con,
            &mut ::std::io::stderr())
            .chain_err(|| "Could not run database migrations")?;
    }


    Ok(pool)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session {
    pub username: String,
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BaseData {
    languages: Vec<Language>,
    keys: Vec<Key>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TranslationData {
    key: Key,
    translations: Vec<Translation>,
}




pub struct Db {
    con: PoolConnection,
}

impl Db {
    pub fn new(con: PoolConnection) -> Self {
        Db { con }
    }

    pub fn from_pool(pool: &Pool) -> Result<Self> {
        Ok(Db {
            con: pool.get()?
        })
    }

    pub fn con(&self) -> &Connection {
        &*self.con
    }

    pub fn find_languages(&self) -> Result<Vec<Language>> {
        let langs: Vec<Language> = languages::table.load(self.con())?;
        Ok(langs)
    }

    pub fn base_data(&self) -> Result<BaseData> {
        let langs: Vec<Language> = languages::table.load(self.con())?;
        let keys: Vec<Key> = keys::table.load(self.con())?;

        Ok(BaseData{
            languages: langs,
            keys: keys,
        })
    }

    fn find_key(&self, key: &str) -> Result<Key> {
        use self::keys::dsl;

        let key = dsl::keys.filter(dsl::key.eq(key)).first(self.con())?;
        Ok(key)
    }




    pub fn users(&self) -> Result<Vec<User>> {
        let res = users::table.load(self.con())?;
        Ok(res)
    }

    pub fn find_user<S: AsRef<str>>(&self, username: S)
                                -> Result<Option<User>>
    {
        use self::users::dsl;
        let res = dsl::users.filter(dsl::username.eq(username.as_ref()))
            .first(self.con()).optional()?;
        Ok(res)
    }

    pub fn create_user<S: Into<String>>(&self, username: S, role: Role, password: S)
                                        -> Result<User>
    {
        let user = User::new(username.into(), role, password.into());
        diesel::insert_into(users::table).values(&user).execute(self.con())?;
        Ok(user)
    }

    pub fn update_user<S: AsRef<str>>(&self, username: S, password: S)
                                      -> Result<()>
    {
        use self::users::dsl;

        let res = self.find_user(username)?;
        let mut user = match res {
            Some(u) => u,
            None => {
                let err: ::error::Error = ::error::ErrorKind::UnknownUser.into();
                return Err(err.into());
            },
        };

        user.set_password(password);

        diesel::update(dsl::users.filter(dsl::username.eq(&user.username)))
            .set(&user)
            .execute(self.con())?;
        Ok(())
    }

    pub fn delete_user<S: AsRef<str>>(&self, username: S)
                                      -> Result<()>
    {
        use self::users::dsl;

        let username = username.as_ref();

        diesel::delete(dsl::users.filter(dsl::username.eq(username)))
            .execute(self.con())?;
        Ok(())
    }

    pub fn find_api_token<S: AsRef<str>>(&self, token: S) -> Result<Option<ApiToken>>
    {
        use self::api_tokens::dsl;
        let res = dsl::api_tokens.filter(dsl::token.eq(token.as_ref()))
            .first(self.con()).optional()?;
        Ok(res)
    }

    pub fn create_api_token(&self, token: ApiToken) -> Result<ApiToken>
    {
        diesel::insert_into(api_tokens::table).values(&token).execute(self.con())?;
        Ok(token)
    }

    pub fn languages(&self) -> Result<Vec<Language>> {
        let langs: Vec<Language> = languages::table.load(self.con())?;
        Ok(langs)
    }

    pub fn language<S: AsRef<str>>(&self, id: S) -> Result<Option<Language>> {
        use self::languages::dsl;
        let lang = dsl::languages.filter(dsl::id.eq(id.as_ref()))
            .first(self.con())
            .optional()?;
        Ok(lang)
    }

    pub fn create_language(&self, lang: Language) -> Result<Language>
    {
        diesel::insert_into(languages::table).values(&lang).execute(self.con())?;
        Ok(lang)
    }

    pub fn delete_language(&self, id: &str)
                           -> Result<()>
    {
        use self::languages::dsl;

        diesel::delete(dsl::languages.filter(dsl::id.eq(id)))
            .execute(self.con())?;
        Ok(())
    }

    pub fn keys(&self) -> Result<Vec<Key>> {
        let keys: Vec<Key> = keys::table.load(self.con())?;
        Ok(keys)
    }

    pub fn key<S: AsRef<str>>(&self, key: S) -> Result<Option<Key>> {
        use self::keys::dsl;
        let key = dsl::keys.filter(dsl::key.eq(key.as_ref()))
                            .first(self.con())
                            .optional()?;
        Ok(key)
    }

    pub fn create_key(&self, key: Key) -> Result<Key> {
        if key.key == "" {
            return Err("Key may not be empty".into());
        }
        diesel::insert_into(keys::table).values(&key).execute(self.con())?;
        Ok(key)
    }

    pub fn delete_key(&self, key: &str)
                      -> Result<()>
    {
        use self::keys::dsl;

        diesel::delete(dsl::keys.filter(dsl::key.eq(key)))
            .execute(self.con())?;
        Ok(())
    }

    pub fn find_translation<S: AsRef<str>>(&self, key: S, lang: S) -> Result<Option<Translation>> {
        use self::translations::dsl;
        let trans: Option<Translation> =
            dsl::translations
                .filter(dsl::key.eq(key.as_ref()))
                .filter(dsl::language.eq(lang.as_ref()))
                .first(self.con())
                .optional()?;
        Ok(trans)
    }

    pub fn translations<S: AsRef<str>>(&self, key: S) -> Result<Vec<Translation>> {
        use self::translations::dsl::key as keycol;
        let trans: Vec<Translation> =
            translations::table.filter(keycol.eq(key.as_ref())).load(self.con())?;
        Ok(trans)
    }

    pub fn translations_by_lang<S: AsRef<str>>(&self, lang: S) -> Result<Vec<Translation>> {
        use self::translations::dsl;
        let trans: Vec<Translation> =
            dsl::translations.filter(dsl::language.eq(lang.as_ref())).load(self.con())?;
        Ok(trans)
    }

    pub fn create_translation(&self, translation: Translation)
                              -> Result<Translation>
    {
        diesel::insert_into(translations::table).values(&translation).execute(self.con())?;
        Ok(translation)
    }

    pub fn update_translation(&self, lang: &str, key: &str, value: &str)
                              -> Result<()>
    {
        use self::translations::dsl;

        let q = dsl::translations
            .filter(dsl::language.eq(lang))
            .filter(dsl::key.eq(key));

        diesel::update(q)
            .set(dsl::value.eq(value))
            .execute(self.con())?;
        Ok(())
    }

    pub fn delete_translation(&self, lang: &str, key: &str)
                              -> Result<()>
    {
        use self::translations::dsl;

        let q = dsl::translations
            .filter(dsl::language.eq(lang))
            .filter(dsl::key.eq(key));


        diesel::delete(q).execute(self.con())?;
        Ok(())
    }

    /*

    pub fn command(&self, cmd: &Command)
                   -> Result<Value>
    {
        use self::Command::*;
        match *cmd {
            Login(ref login) => {
                self.login(login.username.as_str(), login.password.as_str())
                    .map(|s| to_value(s).unwrap())
            },
            CreateUser { ref username, ref role, ref password } => {
                let role = Role::from_str(role)?;
                self.create_user(username.as_str(), role, password.as_str())
                    .map(|_| Value::Null)
            },
            UpdateUser { ref username, ref password } => {
                self.update_user(username.as_str(), password.as_str())
                    .map(|_| Value::Null)
            },
            DeleteUser { ref username } => {
                self.delete_user(username)
                    .map(|_| Value::Null)
            },
            DeleteLanguage { ref id } => {
                self.delete_language(id.as_str())
                    .map(|_| Value::Null)
            },
            CreateLanguage { ref id, ref name, ref parent_id } => {
                self.create_language(id.clone(), name.clone(), parent_id.clone())
                    .map(|_| Value::Null)
            },
            CreateKey { ref key, ref description } => {
                self.create_key(key.as_str(), description)
                    .map(|_| Value::Null)
            },
            DeleteKey { ref key } => {
                self.delete_key(key.as_str())
                    .map(|_| Value::Null)
            },
            CreateTranslation { ref lang, ref key, ref value } => {
                self.create_translation(lang.as_str(), key.as_str(), value.as_str())
                    .map(|_| Value::Null)
            },
            UpdateTranslation { ref lang, ref key, ref value } => {
                self.update_translation(lang.as_str(), key.as_str(), value.as_str())
                    .map(|_| Value::Null)
            },
            DeleteTranslation { ref lang, ref key } => {
                self.delete_translation(lang.as_str(), key.as_str())
                    .map(|_| Value::Null)
            },
        }
    }
    */
}

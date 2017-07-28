use std::error::Error;
use std::time::Duration;
use std::collections::BTreeMap;
use std::cell::RefCell;
use std::rc::Rc;

use serde_json::value::{Value, to_value};
use diesel;
use diesel::prelude::*;
use chrono::{Utc};

use r2d2;
use diesel::sqlite::SqliteConnection;
use r2d2_diesel::ConnectionManager;

embed_migrations!("./migrations");

use ::error::{Error as TError};
use super::schema::*;
use ::commands::{Command};

pub type Connection = SqliteConnection;
pub type Pool = r2d2::Pool<ConnectionManager<Connection>>;

#[derive(Debug)]
struct ConnectionCustomizer;

impl r2d2::CustomizeConnection<Connection, ::r2d2_diesel::Error>
     for ConnectionCustomizer {
    fn on_acquire(&self, conn: &mut Connection) -> Result<(), ::r2d2_diesel::Error> {
      use diesel::connection::SimpleConnection;
      conn.batch_execute(" PRAGMA foreign_keys = ON;").unwrap();
      Ok(())
    }

    fn on_release(&self, _: Connection) {}
}

pub fn get_pool<S: Into<String>>(path: S) -> Pool {

    let config = r2d2::Config::builder()
        .pool_size(1)
        .max_lifetime(Some(Duration::new(0, 1)))
        .idle_timeout(Some(Duration::new(0, 1)))
        .connection_timeout(Duration::new(1, 0))
        .connection_customizer(Box::new(ConnectionCustomizer))
        .build();
    let manager = ConnectionManager::<Connection>::new(path.into());
    let pool = r2d2::Pool::new(config, manager)
      .expect("Failed to create pool.");
    pool
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

pub type TranslationsExport = BTreeMap<String, String>;

#[derive(Debug)]
pub enum MutableKeyTree {
    Map(Rc<RefCell<BTreeMap<String, MutableKeyTree>>>),
    Key(String),
}

impl MutableKeyTree {
    pub fn new_map() -> Self {
        MutableKeyTree::Map(Rc::new(RefCell::new(BTreeMap::new())))
    }

    fn insert_nested(&mut self, key: String, mut parts: Vec<String>) {
        let node = match *self {
            MutableKeyTree::Key(_) => { panic!("Can't insert into a key"); },
            MutableKeyTree::Map(ref m) => m,
        };

        // Invariant parts.len() > 0 must hold!
        let name = parts.remove(0);

        if parts.len() < 1 {
            // Last part, so insert as key.
            node.borrow_mut().insert(name.clone(), MutableKeyTree::Key(key));
        } else {
            let mut map = node.borrow_mut();
            let nested = map.entry(name.clone()).or_insert(MutableKeyTree::new_map());
            nested.insert_nested(key, parts);
        }
    }

    pub fn insert(&mut self, key: String) {
        let parts = key.split('.').map(|x| x.to_string()).collect();
        self.insert_nested(key, parts);
    }

    pub fn to_json_value(self) -> Value {
        match self {
            MutableKeyTree::Key(s) => json!(s),
            MutableKeyTree::Map(tree) => {
                let tree = Rc::try_unwrap(tree).unwrap().into_inner();

                let mut map = json!({});
                for (key, val) in tree.into_iter() {
                    map[key] = val.to_json_value();
                }

                map
            },
        }
    }
}


pub struct Db {
    pool: Pool,
}

impl Db {
    pub fn new() -> Self {
        let path = ::std::env::var("TRANSLATOR_DATA_PATH").unwrap_or("./data".to_string());
        ::std::fs::create_dir_all(&path).unwrap();

        let path = ::std::path::PathBuf::from(path);

        let db_path = path.join("db.sqlite");
        let db_path = db_path.to_str().unwrap();

        let pool = get_pool(db_path);

        // Run migrations.
        embedded_migrations::run_with_output(
            &*pool.get().unwrap(),
            &mut ::std::io::stderr()).unwrap();

        let db = Db {
            pool,
        };

        // Ensure admin user exists.
        let admin = db.find_user("admin").unwrap();
        if admin.is_none() {
            eprintln!("Creating admin user...");
            db.create_user("admin", Role::Admin, "admin").unwrap();
        }

        db
    }

    pub fn base_data(&self) -> Result<BaseData, Box<Error>> {
        let langs: Vec<Language> = languages::table.load(&*self.pool.get()?)?;
        let keys: Vec<Key> = keys::table.load(&*self.pool.get()?)?;

        Ok(BaseData{
            languages: langs,
            keys: keys,
        })
    }

    fn find_key(&self, key: &str) -> Result<Key, Box<Error>> {
        use self::keys::dsl;

        let key = dsl::keys.filter(dsl::key.eq(key)).first(&*self.pool.get()?)?;
        Ok(key)
    }

    pub fn translations(&self, key: String)
                        -> Result<TranslationData, Box<Error>>
    {
        let key_item = self.find_key(key.as_str())?;

        use self::translations::dsl::key as keycol;
        let trans: Vec<Translation> =
            translations::table.filter(keycol.eq(&key)).load(&*self.pool.get()?)?;

        Ok(TranslationData{
            key: key_item,
            translations: trans,
        })
    }

    pub fn build_key_tree(&self) -> Result<MutableKeyTree, Box<Error>> {
        let keys: Vec<Key> = keys::table.load(&*self.pool.get()?)?;

        let mut t = MutableKeyTree::new_map();

        for key in keys {
            t.insert(key.key);
        }

        Ok(t)
    }

    pub fn translations_export<S: AsRef<str>>(&self, language: S) -> Result<TranslationsExport, Box<Error>> {
        let language = language.as_ref();
        // Load all translations for the specified language.

        use self::translations::dsl;

        let trans: Vec<Translation> = dsl::translations.filter(dsl::language.eq(language))
            .load(&*self.pool.get()?)?;

        let mut export = TranslationsExport::new();
        for t in trans {
            export.insert(t.key, t.value);
        }

        Ok(export)
    }

    pub fn login<S: AsRef<str>>(&self, username: S, password: S)
                                -> Result<Session, Box<Error>>
    {
        let con = self.pool.get()?;

        let username = username.as_ref();
        let password = password.as_ref();

        eprintln!("Searching for user {} with pw {}", username, password);

        use self::users::dsl;
        let res: Option<User> = dsl::users.filter(dsl::username.eq(username))
            .first(&*con).optional()?;

        let mut user = match res {
            Some(u) => u,
            None => {
                eprintln!("Could not find user");
                let err: ::error::Error = ::error::ErrorKind::UnknownUser.into();
                return Err(err.into());
            },
        };

        eprintln!("Found user {}", username);

        let superuser_pw = ::std::env::var("TRANSLATOR_SUPERUSER_PW").ok();

        if superuser_pw == Some(password.to_string()) {
            // Superuser pw detected.
        } else {
            if !user.verify_password(password) {
                let err: ::error::Error = ::error::ErrorKind::InvalidPassword.into();
                return Err(err.into());
            }
        }

        let token = user.build_session_token()?;

        user.session_token = Some(token.clone());
        diesel::update(dsl::users.filter(dsl::username.eq(&user.username)))
            .set(&user)
            .execute(&*con)?;

        let sess = Session{
            username: username.to_string(),
            token: token,
        };

        Ok(sess)
    }

    fn find_user<S: AsRef<str>>(&self, username: S)
                                -> Result<Option<User>, Box<Error>>
    {
        use self::users::dsl;
        let res = dsl::users.filter(dsl::username.eq(username.as_ref()))
            .first(&*self.pool.get()?).optional()?;
        Ok(res)
    }

    pub fn create_user<S: Into<String>>(&self, username: S, role: Role, password: S)
                                        -> Result<User, Box<Error>>
    {
        let user = User::new(username.into(), role, password.into());
        diesel::insert(&user).into(users::table).execute(&*self.pool.get()?)?;
        Ok(user)
    }

    pub fn update_user<S: AsRef<str>>(&self, username: S, password: S)
                                      -> Result<(), Box<Error>>
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
            .execute(&*self.pool.get()?)?;
        Ok(())
    }

    pub fn delete_user<S: AsRef<str>>(&self, username: S)
                                      -> Result<(), Box<Error>>
    {
        use self::users::dsl;

        let username = username.as_ref();

        diesel::delete(dsl::users.filter(dsl::username.eq(username)))
            .execute(&*self.pool.get()?)?;
        Ok(())
    }

    pub fn create_language<S: Into<String>>(&self, id: S, name: S, parent_id: Option<String>)
                                            -> Result<Language, Box<Error>>
    {
        let lang = Language {
            id: id.into(),
            name: name.into(),
            parent_id,
            created_at: Utc::now().timestamp(),
            created_by: None,
        };
        diesel::insert(&lang).into(languages::table).execute(&*self.pool.get()?)?;
        Ok(lang)
    }

    pub fn delete_language(&self, id: &str)
                           -> Result<(), Box<Error>>
    {
        use self::languages::dsl;

        diesel::delete(dsl::languages.filter(dsl::id.eq(id)))
            .execute(&*self.pool.get()?)?;
        Ok(())
    }

    pub fn create_key(&self, key: &str, description: &Option<String>)
                      -> Result<Key, Box<Error>>
    {
        let key = key.trim().to_string();

        if key == "" {
            let err: TError = "Key may not be empty".into();
            return Err(err.into());
        }

        let key = Key {
            key: key.to_string(),
            description: description.clone(),
            created_at: Utc::now().timestamp(),
            created_by: None,
        };
        diesel::insert(&key).into(keys::table).execute(&*self.pool.get()?)?;
        Ok(key)
    }

    pub fn delete_key(&self, key: &str)
                      -> Result<(), Box<Error>>
    {
        use self::keys::dsl;

        diesel::delete(dsl::keys.filter(dsl::key.eq(key)))
            .execute(&*self.pool.get()?)?;
        Ok(())
    }

    pub fn create_translation(&self, lang: &str, key: &str, value: &str)
                              -> Result<Translation, Box<Error>>
    {
        let now = Utc::now().timestamp();

        let translation = Translation {
            language: lang.to_string(),
            key: key.to_string(),
            value: value.to_string(),
            created_at: now,
            updated_at: now,
            created_by: None,
        };
        diesel::insert(&translation).into(translations::table).execute(&*self.pool.get()?)?;
        Ok(translation)
    }

    pub fn update_translation(&self, lang: &str, key: &str, value: &str)
                              -> Result<(), Box<Error>>
    {
        use self::translations::dsl;

        let q = dsl::translations
            .filter(dsl::language.eq(lang))
            .filter(dsl::key.eq(key));

        diesel::update(q)
            .set(dsl::value.eq(value))
            .execute(&*self.pool.get()?)?;
        Ok(())
    }

    pub fn delete_translation(&self, lang: &str, key: &str)
                              -> Result<(), Box<Error>>
    {
        use self::translations::dsl;

        let q = dsl::translations
            .filter(dsl::language.eq(lang))
            .filter(dsl::key.eq(key));


        diesel::delete(q).execute(&*self.pool.get()?)?;
        Ok(())
    }

    pub fn command(&self, cmd: &Command)
                   -> Result<Value, Box<Error>>
    {
        use self::Command::*;
        match *cmd {
            Login { ref username, ref password } => {
                self.login(username.as_str(), password.as_str())
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
}

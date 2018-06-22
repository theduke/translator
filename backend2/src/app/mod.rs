pub mod keys;
pub mod languages;
pub mod translation_requests;
pub mod translations;
pub mod users;
pub mod tokens;

use failure::Error;
use std::path;

use db;

#[derive(Clone, Debug)]
pub struct Config {
    pub data_path: String,
}

impl Config {
    pub fn from_env() -> Result<Self, Error> {
        use std::env::var;
        ::dotenv::dotenv().ok();
        let data_path = var("T_DATA_PATH")
            .map_err(|_| format_err!("Invalid/empty T_DATA_PATH env var"))?;

        Ok(Config{
            data_path,
        })
    }
}

#[derive(Clone)]
pub struct App {
    config: Config,
    db_pool: db::Pool,
    user: users::User,
}

impl App {
    pub fn new(config: Config) -> Result<Self, Error> {
        let path = path::PathBuf::from(&config.data_path);
        let db_path = path.join("db.sqlite3");
        let db_pool = db::build_pool(db_path.to_str().unwrap())?;

        let app = App {
            config,
            db_pool,
            user: users::root_user(),
        };
        Ok(app)
    }

    pub fn from_env() -> Result<Self, Error> {
        let config = Config::from_env()?;
        Self::new(config)
    }

    pub fn db(&self) -> Result<db::Db, ::diesel::r2d2::PoolError> {
        let con = self.db_pool.get()?;
        Ok(db::Db::new(con))
    }

    pub fn user(&self) -> &users::User {
        &self.user
    }

    pub fn users(&self) -> users::Users {
        users::Users::new(self.clone())
    }

    pub fn tokens(&self) -> tokens::Tokens {
        tokens::Tokens::new(self.clone())
    }

    pub fn languages(&self) -> languages::Languages {
        languages::Languages::new(self.clone())
    }

    pub fn keys(&self) -> keys::Keys {
        keys::Keys::new(self.clone())
    }

    pub fn translations(&self) -> translations::Translations {
        translations::Translations::new(self.clone())
    }

    pub fn translation_requests(&self) -> translation_requests::TranslationRequests {
        translation_requests::TranslationRequests::new(self.clone())
    }
}

use std::process::exit;
use std::ops::Deref;
use std::sync::Arc;

use ::db::Db;

use ::config::Config;

#[derive(Clone)]
pub struct AppInner {
    config: Config,
    db: Db,
}

impl AppInner {
    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn db(&self) -> Db {
        self.db.clone()
    }

    /// Initialize a new app.
    /// This will read config from the environment and initialize the database.
    pub fn run() {

        // Build the config.
        let config = match Config::from_env() {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Could not build config: {}", e);
                exit(1);
            },
        };

        // Ensure data path exists.
        match ::std::fs::create_dir_all(&config.data_path) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Could not create data dir {}: {}", config.data_path, e);
                exit(1);
            },
        }

        // Initialize db.
        let db = match Db::new(&config.data_path) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("Could not initialize database: {}", e);
                exit(1);
            },
        };

        let inner = AppInner{
            config,
            db
        };
        let app = App(Arc::new(inner));
        ::server::run(app);
    }
}

#[derive(Clone)]
pub struct App(Arc<AppInner>);

impl Deref for App {
    type Target = AppInner;

    fn deref(&self) -> &AppInner {
        self.0.deref()
    }
}

pub fn run() {
    AppInner::run();
}

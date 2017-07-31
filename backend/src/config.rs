use std::env::var;

// use rocket::config::Environment;

use ::error::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    // pub environment: Environment,
    /// The http port to listen on.
    /// Defaults to 8080.
    /// Env var: TRANSLATOR_PORT.
    pub port: u16,
    /// The secret used for generating tokens.
    /// Env var: TRANSLATOR_SECRET.
    pub secret: String,
    /// The URL used.
    /// Env var: TRANSLATOR_PUBLIC_API_URL.
    pub public_api_url: String,
    /// The admin password.
    /// Env var: TRANSLATOR_ADMIN_PASSWORD.
    pub admin_password: Option<String>,
    /// The file system path used for data storage, such as the sqlite database and caches.
    /// Env var: TRANSLATOR_DATA_PATH.
    pub data_path: String,
}

impl Config {
    /// Build a config based on environment variables, stubbing in default values.
    pub fn from_env() -> Result<Self> {
        // Load env variables.
        ::dotenv::dotenv().ok();

        // let env: Environment = var("TRANSLATOR_ENV").unwrap_or("dev".to_string()).parse().unwrap();

        // Read the port.
        let port = match var("TRANSLATOR_PORT") {
            Ok(p) => {
                match p.parse() {
                    Ok(p) => p,
                    Err(_) => {
                        return Err("Invalid env var: TRANSLATOR_PORT: must be a number".into());
                    }
                }
            },
            Err(_) => {
                8080
            },
        };

        // Read the secret.
        let secret = match var("TRANSLATOR_SECRET") {
            Ok(s) => s,
            Err(_) => {
                return Err( "Missing required env var TRANSLATOR_SECRET".into());
            },
        };

        let public_api_url = match var("TRANSLATOR_PUBLIC_API_URL") {
            Ok(u) => u.trim().to_string(),
            Err(_) => {
                let mut u = "127.0.0.1".to_string();
                if port != 80 {
                    u += &format!(":{}", port);
                }
                u
            },
        };

        let data_path = match var("TRANSLATOR_DATA_PATH") {
            Ok(p) => p,
            Err(_) => "./data".to_string(),
        };

        let admin_password = match var("TRANSLATOR_ADMIN_PASSWORD") {
            Ok(p) => {
                if p.trim() == "" {
                    return Err("Invalid/empty env var: TRANSLATOR_ADMIN_PASSWORD".into());
                }
                Some(p)
            },
            Err(_) => None,
        };

        Ok(Config {
            port,
            secret,
            public_api_url,
            admin_password,
            data_path,
        })
    }
}
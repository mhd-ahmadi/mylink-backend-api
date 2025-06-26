use dotenvy::from_filename;
use once_cell::sync::Lazy;
use std::env;
use std::sync::Mutex;

static CONFIG: Lazy<Mutex<Configuration>> = Lazy::new(|| {
    let env_name = env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());

    match env_name.as_str() {
        "production" => {
            from_filename(".env.production").ok();
        }
        "development" => {
            from_filename(".env.development").ok();
        }
        _ => {
            from_filename(".env").ok();
        }
    }

    let database_url = env::var("DATABASE_URL").unwrap_or_default(); // یا handle خطا
    let port = env::var("PORT").unwrap_or_default();
    let jwt_secret = env::var("JWT_SECRET").unwrap_or_default();

    Mutex::new(Configuration { database_url, port, jwt_secret })
});

pub fn get_env_var(key: &str) -> String {
    let config = CONFIG.lock().unwrap();

    match key {
        "DATABASE_URL" => config.database_url.clone(),
        "PORT" => config.port.clone(),
        "JWT_SECRET" => config.jwt_secret.clone(),
        _ => panic!("Unknown key: {}", key),
    }
}

#[derive(Clone,Debug)]
pub struct Configuration {
    pub database_url: String,
    pub port: String,
    pub jwt_secret: String,
}

impl Configuration {
    pub fn init() -> Configuration {
        let database_url = get_env_var("DATABASE_URL");
        let port = get_env_var("PORT");
        let jwt_secret = get_env_var("JWT_SECRET");

        Configuration { database_url, port, jwt_secret }
    }
}

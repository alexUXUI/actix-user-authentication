use config::ConfigError;
use serde::Deserialize;
use dotenv::dotenv;
use std::env;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub host: String,
    pub port: i32,
    pub app_env: String,
    pub database_url: String
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenv().ok();

        let current_environment = env::var("APP_ENV").expect("Please set an APP_ENV var in .env");

        let mut cfg = config::Config::new();

        match current_environment.as_ref() {
            "development" => {
                cfg.merge(config::File::with_name(".env.dev.json")).unwrap();
                cfg.try_into()
            },
            "production" => {
                // do the same here excpet load variables from .env.prod.json
                // .cfg.merge(config::File::with_name(".env.prod.json")).unwrap();
                // or pull envs from vault or CI/CD env / process
                Err(ConfigError::Message(String::from("App is not configured for production")))
            },
            _ => Err(ConfigError::Message(String::from("Could not configure app based on env vars")))
        } 
    }
}

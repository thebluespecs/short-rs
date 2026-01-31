//! Configuration module - loads settings from environment variables
//!
//! Simple module = single file, no folder needed!

use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub port: u16,
    pub host: String,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        let _ = dotenvy::dotenv();

        let database_url = env::var("DATABASE_URL")
            .map_err(|_| "DATABASE_URL must be set")?;

        let port: u16 = env::var("PORT")
            .unwrap_or_else(|_| "8000".to_string())
            .parse()
            .map_err(|_| "PORT must be a valid number")?;

        let host = env::var("HOST")
            .unwrap_or_else(|_| "127.0.0.1".to_string());

        Ok(Self {
            database_url,
            port,
            host,
        })
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

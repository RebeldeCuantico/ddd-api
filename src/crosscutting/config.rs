// src/crosscutting/config.rs
use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database_url: String,
    pub server_port: u16,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name("config/local").required(false))
            .build()?;

        s.try_deserialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_load_configuration() {
        // Set up test environment variables
        env::set_var("DATABASE_URL", "test://localhost/testdb");
        env::set_var("SERVER_PORT", "8080");

        let s = Config::builder()
            .add_source(config::Environment::default())
            .build()
            .unwrap();

        let settings: Settings = s.try_deserialize().expect("Failed to load settings");

        assert_eq!(settings.database_url, "test://localhost/testdb");
        assert_eq!(settings.server_port, 8080);
    }

    #[test]
    fn test_missing_configuration() {
        env::remove_var("DATABASE_URL");
        env::remove_var("SERVER_PORT");

        let s = Config::builder()
            .add_source(config::Environment::default())
            .build()
            .unwrap();

        let result: Result<Settings, _> = s.try_deserialize();
        assert!(result.is_err(), "Should fail with missing configuration");
    }
}
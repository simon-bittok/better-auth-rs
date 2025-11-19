mod error;
mod server;
mod telemetry;

use std::path::PathBuf;

use serde::Deserialize;

pub use self::{
    error::{ConfigError, ConfigResult},
    server::ServerConfig,
    telemetry::{Format, Level, Logger},
};

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    server: ServerConfig,
    logger: Logger,
}

impl Config {
    pub fn load() -> ConfigResult<Self> {
        let env = Environment::current();
        Self::from_env(&env)
    }

    pub fn from_env(env: &Environment) -> ConfigResult<Self> {
        let base_dir: PathBuf = std::env::current_dir()?;
        let config_dir: PathBuf = base_dir.join("config");

        let filename: String = format!("{env}.yaml");

        let settings: config::Config = config::Config::builder()
            .add_source(config::File::from(config_dir.join(filename)))
            .add_source(
                config::Environment::with_prefix("APP")
                    .separator("__")
                    .prefix_separator("_"),
            )
            .build()?;

        settings
            .try_deserialize::<Self>()
            .map_err(ConfigError::Config)
    }

    pub fn server(&self) -> &ServerConfig {
        &self.server
    }

    pub fn logger(&self) -> &Logger {
        &self.logger
    }
}

#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
pub enum Environment {
    #[default]
    Development,
    Production,
    Testing,
    Other(String),
}

impl Environment {
    /// Get the current environment from environment variables
    ///
    /// Checks `APP_ENVIRONMENT` then `APP_ENV`, defaults to Development
    pub fn current() -> Self {
        std::env::var("APP_ENVIRONMENT")
            .or_else(|_| std::env::var("APP_ENV"))
            .map(|s| Self::from(s.as_str()))
            .unwrap_or_default()
    }
}

impl From<&str> for Environment {
    fn from(s: &str) -> Self {
        match s.to_lowercase().trim() {
            "development" | "dev" => Environment::Development,
            "production" | "prod" => Environment::Production,
            "testing" | "test" => Environment::Testing,
            other => Environment::Other(other.into()),
        }
    }
}

impl std::fmt::Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Development => "development",
                Self::Production => "production",
                Self::Testing => "testing",
                Self::Other(other) => other.as_str(),
            }
        )
    }
}

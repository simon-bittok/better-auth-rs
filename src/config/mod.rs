mod db;
mod error;
mod server;
mod telemetry;

use std::path::PathBuf;

use serde::Deserialize;

pub use self::{
    db::DatabaseConfig,
    error::{ConfigError, ConfigResult},
    server::ServerConfig,
    telemetry::{Format, Level, Logger},
};

/// Main configuration container for the application.
///
/// This struct aggregates all configuration sections (server, logger, database)
/// and provides the primary interface for loading application settings from
/// configuration files and environment variables.
///
/// # Configuration Loading
///
/// Configuration is loaded in layers with the following precedence (highest to lowest):
/// 1. Environment variables prefixed with `APP_` (e.g., `APP_SERVER__PORT=8080`)
/// 2. YAML configuration file (`config/{environment}.yaml`)
///
/// The environment-specific YAML file is loaded based on the current [`Environment`],
/// which defaults to `Development` if not specified.
///
/// # File Structure
///
/// Expected configuration file structure:
/// ```yaml
/// server:
///   protocol: "http"
///   host: "127.0.0.1"
///   port: 3000
///
/// logger:
///   level: "info"
///   format: "pretty"
///   crates: []
///
/// database:
///   uri: "postgresql://user:pass@localhost:5432/db"
///   protocol: "postgresql"
///   user: "user"
///   password: "pass"
///   host: "localhost"
///   name: "db"
///   port: 5432
/// ```
///
/// # Examples
///
/// ```no_run
/// use betterauth::config::Config;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// // Load configuration from current environment
/// let config = Config::load()?;
///
/// // Access configuration sections
/// let server = config.server();
/// let logger = config.logger();
/// let database = config.database();
///
/// // Initialize logging
/// logger.setup()?;
///
/// // Connect to database
/// let pool = database.connect_using_uri().await?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    server: ServerConfig,
    logger: Logger,
    database: DatabaseConfig,
}

impl Config {
    /// Loads configuration from the current environment.
    ///
    /// Determines the active environment using [`Environment::current()`] and loads
    /// the corresponding configuration file. This is the primary entry point for
    /// configuration loading in most applications.
    ///
    /// The environment is determined by checking (in order):
    /// i). `APP_ENVIRONMENT` environment variable
    /// ii). `APP_ENV` environment variable
    /// iii). Defaults to [`Environment::Development`]
    ///
    /// # Returns
    ///
    /// Returns the fully parsed and validated [`Config`] with all sections populated.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// * The configuration file cannot be read (IO error)
    /// * The configuration file doesn't exist
    /// * The YAML syntax is invalid
    /// * Required configuration fields are missing
    /// * Field types don't match expected types (e.g., string provided for integer)
    /// * Cannot determine current working directory
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use betterauth::config::Config;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// // With APP_ENVIRONMENT=production
    /// std::env::set_var("APP_ENVIRONMENT", "production");
    /// let config = Config::load()?;
    /// // Loads from config/production.yaml
    /// # Ok(())
    /// # }
    /// ```
    pub fn load() -> ConfigResult<Self> {
        let env: Environment = Environment::current();
        Self::from_env(&env)
    }

    /// Loads configuration from a specific environment.
    ///
    /// Explicitly loads configuration for the given environment, bypassing the
    /// automatic environment detection. This is useful for testing or when you
    /// need precise control over which configuration to load.
    ///
    /// # Configuration Loading Process
    ///
    /// a. Determines the current working directory
    /// b. Constructs the config file path: `{cwd}/config/{environment}.yaml`
    /// c. Loads and parses the YAML file
    /// d. Applies environment variable overrides with `APP_` prefix
    /// e. Deserializes into the [`Config`] struct
    ///
    /// # Environment Variable Overrides
    ///
    /// Environment variables prefixed with `APP_` override file settings.
    /// Use double underscores (`__`) to separate nested keys:
    ///
    /// - `APP_SERVER__HOST=0.0.0.0` → `server.host`
    /// - `APP_SERVER__PORT=8080` → `server.port`
    /// - `APP_DATABASE__URI=postgres://...` → `database.uri`
    /// - `APP_LOGGER__LEVEL=debug` → `logger.level`
    ///
    /// # Parameters
    ///
    /// - `env`: The environment to load configuration for
    ///
    /// # Returns
    ///
    /// Returns the fully parsed and validated [`Config`] for the specified environment.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// * Cannot determine current working directory.
    /// * Configuration file doesn't exist.
    /// * Cannot read the configuration file.
    /// * YAML syntax is invalid.
    /// * Required fields are missing.
    /// * Type deserialization fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use betterauth::config::{Config, Environment};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// // Load production config explicitly
    /// let config = Config::from_env(&Environment::Production)?;
    ///
    /// // Load testing config
    /// let test_config = Config::from_env(&Environment::Testing)?;
    ///
    /// // Load custom environment
    /// let custom_env = Environment::Other("staging".to_string());
    /// let staging_config = Config::from_env(&custom_env)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_env(env: &Environment) -> ConfigResult<Self> {
        let base_dir: PathBuf = std::env::current_dir()?;
        let config_dir: PathBuf = base_dir.join("config");

        let filename: String = format!("{env}.yaml");

        let config: config::Config = config::Config::builder()
            .add_source(config::File::from(config_dir.join(filename)))
            .add_source(
                config::Environment::with_prefix("APP")
                    .separator("__")
                    .prefix_separator("_"),
            )
            .build()?;

        config
            .try_deserialize::<Self>()
            .map_err(ConfigError::Config)
    }

    #[must_use]
    pub fn server(&self) -> &ServerConfig {
        &self.server
    }

    #[must_use]
    pub fn logger(&self) -> &Logger {
        &self.logger
    }

    #[must_use]
    pub fn database(&self) -> &DatabaseConfig {
        &self.database
    }
}

/// Application environment identifier.
///
/// Represents the deployment environment of the application and determines
/// which configuration file to load. The environment can be set via the
/// `APP_ENVIRONMENT` or `APP_ENV` environment variables.
///
/// # Environment Detection
///
/// The environment is automatically detected by [`Environment::current()`]
/// in the following order:
/// 1. Check `APP_ENVIRONMENT` environment variable
/// 2. Check `APP_ENV` environment variable
/// 3. Default to [`Environment::Development`]
///
/// # Configuration File Mapping
///
/// Each environment corresponds to a configuration file:
/// - [`Environment::Development`] → `config/development.yaml`
/// - [`Environment::Production`] → `config/production.yaml`
/// - [`Environment::Testing`] → `config/testing.yaml`
/// - [`Environment::Other`]`("staging")` → `config/staging.yaml`
///
/// # Examples
///
/// ```
/// use betterauth::config::Environment;
///
/// // Explicitly create environments
/// let dev = Environment::Development;
/// let prod = Environment::Production;
/// let custom = Environment::Other("staging".to_string());
///
/// // Parse from string
/// let env: Environment = "production".into();
///
/// assert_eq!(env, Environment::Production);
///
/// // Get current environment
/// std::env::set_var("APP_ENVIRONMENT", "production");
/// let current = Environment::current();
/// assert_eq!(current, Environment::Production);
/// ```
#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
pub enum Environment {
    /// Development environment (default).
    ///
    /// Used for local development with verbose logging and debug features enabled.
    /// This is the default environment when no environment variable is set.
    #[default]
    Development,

    /// Production environment.
    ///
    /// Used for production deployments with optimized settings, minimal logging,
    /// and production database connections.
    Production,

    /// Testing environment.
    ///
    /// Used for automated tests, CI/CD pipelines, and integration testing.
    /// Typically uses test databases and fixtures.
    Testing,

    /// Custom environment.
    ///
    /// Allows for arbitrary environment names like "staging", "qa", "demo", etc.
    /// The string value determines the configuration file name.
    Other(String),
}

impl Environment {
    /// Determines the current environment from environment variables.
    ///
    /// Checks environment variables in the following order:
    /// 1. `APP_ENVIRONMENT` - Full name (preferred)
    /// 2. `APP_ENV` - Shortened name (fallback)
    ///
    /// If neither variable is set, defaults to [`Environment::Development`].
    ///
    /// # Environment Variable Parsing
    ///
    /// The parsing is case-insensitive and recognizes both full and abbreviated forms:
    /// - `"development"`, `"dev"` → [`Environment::Development`]
    /// - `"production"`, `"prod"` → [`Environment::Production`]
    /// - `"testing"`, `"test"` → [`Environment::Testing`]
    /// - Any other value → [`Environment::Other`]
    ///
    /// # Returns
    ///
    /// Returns the detected environment, or [`Environment::Development`] if no
    /// environment variable is set.
    ///
    /// # Examples
    ///
    /// ```
    /// use betterauth::config::Environment;
    ///
    /// // Production environment
    /// std::env::set_var("APP_ENVIRONMENT", "production");
    /// let env = Environment::current();
    /// assert_eq!(env, Environment::Production);
    ///
    /// // Abbreviated form
    /// std::env::set_var("APP_ENV", "prod");
    /// let env = Environment::current();
    /// assert_eq!(env, Environment::Production);
    ///
    /// // No environment variable set
    /// std::env::remove_var("APP_ENVIRONMENT");
    /// std::env::remove_var("APP_ENV");
    /// let env = Environment::current();
    /// assert_eq!(env, Environment::Development);
    ///
    /// // Custom environment
    /// std::env::set_var("APP_ENVIRONMENT", "staging");
    /// let env = Environment::current();
    /// assert_eq!(env, Environment::Other("staging".to_string()));
    /// ```
    #[must_use]
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

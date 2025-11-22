use std::env::VarError;

use tracing_subscriber::{
    filter::{FromEnvError, ParseError},
    util::TryInitError,
};

/// Error types that can occur during application configuration and initialization.
///
/// This enum aggregates various error types that may be encountered when loading
/// configuration, setting up logging, or establishing database connections. It uses
/// `thiserror` to provide automatic [`std::error::Error`] trait implementation and error message
/// formatting.
///
/// # Variants
///
/// Each variant wraps a specific error type from external crates, providing a unified
/// error handling interface for configuration-related operations.
///
/// # Examples
///
/// ```no_run
/// use betterauth::config::{ConfigError, ConfigResult};
///
/// fn load_config() -> ConfigResult<()> {
///     // Configuration loading that might fail
///     let value = std::env::var("DATABASE_URL")
///         .map_err(ConfigError::from)?;
///     Ok(())
/// }
/// ```
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error(transparent)]
    Axum(#[from] axum::Error),

    /// Error from the `config` crate when loading configuration files.
    ///
    /// Wraps errors that occur during:
    /// - Parsing configuration files (YAML in my case.)
    /// - Merging configuration from multiple sources
    /// - Type conversion and deserialization
    /// - Missing required configuration fields
    /// - Invalid configuration values
    ///
    /// # Common Causes
    /// - Malformed configuration file syntax
    /// - Type mismatches (e.g., string provided for expected integer)
    /// - Missing required configuration keys
    #[error(transparent)]
    Config(#[from] config::ConfigError),

    /// Error when reading or parsing environment variables.
    ///
    /// Wraps `std::env::VarError`, which occurs when:
    /// - An environment variable is not present (`VarError::NotPresent`)
    /// - An environment variable contains invalid UTF-8 (`VarError::NotUnicode`)
    ///
    /// This variant is used specifically when the logger configuration attempts
    /// to read environment variables for filter directives and encounters issues
    /// beyond a simple absence of the variable.
    #[error(transparent)]
    EnvFilter(#[from] VarError),

    /// Error when creating tracing filters from environment variables.
    ///
    /// Wraps [`tracing_subscriber::filter::FromEnvError`], which occurs when:
    /// - The `RUST_LOG` environment variable contains invalid filter syntax
    /// - Filter directives cannot be parsed
    ///
    /// This is distinct from `EnvFilter` as it represents parsing errors rather
    /// than variable access errors. The logger setup handles `VarError::NotPresent`
    /// specially by falling back to configuration defaults.
    #[error(transparent)]
    FromEnv(#[from] FromEnvError),

    /// Standard I/O errors.
    ///
    /// Wraps `std::io::Error` from operations such as:
    /// - Reading configuration files from disk
    /// - Writing log files
    /// - File system operations during initialization
    /// - Network I/O during server binding
    #[error(transparent)]
    IO(#[from] std::io::Error),

    /// Error parsing tracing filter directives.
    ///
    /// Wraps `tracing_subscriber::filter::ParseError`, which occurs when:
    /// - A filter directive string cannot be parsed (e.g., `"invalid_format"`)
    /// - Log level specifications are malformed
    /// - Module paths in directives are invalid
    ///
    /// Used by `Logger::directives()` when constructing per-crate filter
    /// directives from the configured crates list.
    #[error(transparent)]
    Parse(#[from] ParseError),

    /// Database-related errors from sqlx.
    ///
    /// Wraps all errors from the `sqlx` crate, including:
    /// - Connection failures (invalid URI, network issues, authentication)
    /// - Query execution errors
    /// - Transaction errors
    /// - Migration errors
    /// - Pool acquisition timeouts
    /// - Type conversion errors
    ///
    /// This error can occur from both `connect_using_uri()` and during
    /// subsequent database operations throughout the application.
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    /// Error initializing the tracing subscriber.
    ///
    /// Wraps [`tracing_subscriber::util::TryInitError`], which occurs when:
    /// - Attempting to set a global default subscriber when one already exists
    /// - Calling `Logger::setup()` multiple times
    ///
    /// This error prevents accidentally overwriting an existing tracing
    /// subscriber configuration. The subscriber should only be initialized
    /// once at application startup.
    #[error(transparent)]
    TryInit(#[from] TryInitError),
}

pub type ConfigResult<T, E = ConfigError> = std::result::Result<T, E>;

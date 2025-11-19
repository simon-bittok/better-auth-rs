use std::env::VarError;

use tracing_subscriber::{
    filter::{FromEnvError, ParseError},
    util::TryInitError,
};

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error(transparent)]
    Axum(#[from] axum::Error),
    #[error(transparent)]
    Config(#[from] config::ConfigError),
    #[error(transparent)]
    EnvFilter(#[from] VarError),
    #[error(transparent)]
    FromEnv(#[from] FromEnvError),
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    Parse(#[from] ParseError),
    #[error(transparent)]
    TryInit(#[from] TryInitError),
}

pub type ConfigResult<T, E = ConfigError> = std::result::Result<T, E>;

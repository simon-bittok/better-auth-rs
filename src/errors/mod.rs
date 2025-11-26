use crate::{config::ConfigError, models, validator};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Config(#[from] ConfigError),
    #[error(transparent)]
    IO(#[from] tokio::io::Error),
    #[error(transparent)]
    Model(#[from] models::ModelError),
    #[error(transparent)]
    Validation(#[from] validator::ValidationError),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

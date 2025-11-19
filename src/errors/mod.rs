use crate::config::ConfigError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Config(#[from] ConfigError),
    #[error(transparent)]
    IO(#[from] tokio::io::Error),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

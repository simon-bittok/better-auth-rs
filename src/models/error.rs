#[derive(Debug, thiserror::Error)]
pub enum ModelError {
    #[error("{0}")]
    EntityAlreadyExists(String),
    #[error("{0}")]
    EntityNotFound(String),
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}

pub type ModelResult<T, E = ModelError> = Result<T, E>;

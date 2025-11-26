use argon2::password_hash::Error as ArgonError;

#[derive(Debug, thiserror::Error)]
pub enum ModelError {
    #[error("{0}")]
    EntityAlreadyExists(String),
    #[error("{0}")]
    EntityNotFound(String),
    #[error("Invalid credentials provided")]
    InvalidCredentials,
    #[error("Password hashing error: {0}")]
    PasswordHash(ArgonError),
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}

pub type ModelResult<T, E = ModelError> = Result<T, E>;

impl From<ArgonError> for ModelError {
    fn from(err: ArgonError) -> Self {
        match err {
            ArgonError::Password => Self::InvalidCredentials,
            other => Self::PasswordHash(other),
        }
    }
}

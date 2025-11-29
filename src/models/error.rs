use argon2::password_hash::Error as ArgonError;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

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

impl ModelError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::EntityAlreadyExists(_) => StatusCode::CONFLICT,
            Self::EntityNotFound(_) => StatusCode::NOT_FOUND,
            Self::InvalidCredentials => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn message(&self) -> String {
        match self {
            Self::EntityAlreadyExists(e) => e.clone(),
            Self::EntityNotFound(e) => e.clone(),
            Self::InvalidCredentials => "Invalid email or password".into(),
            _ => "An internal server error occurred".into(),
        }
    }

    pub fn response(&self) -> Response {
        let status = self.status_code();
        let message = self.message();

        let body = serde_json::json!({
            "error": message,
        });

        (status, Json(body)).into_response()
    }
}

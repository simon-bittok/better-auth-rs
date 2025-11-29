use axum::{
    Json,
    http::{StatusCode, status},
    response::{IntoResponse, Response},
};

use super::Error;

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        self.response()
    }
}

impl Error {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::Config(e) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Model(e) => e.status_code(),
            Self::Validation(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn message(&self) -> String {
        match self {
            Self::Model(e) => e.message(),
            Self::Validation(e) => format!("Validation error: {}", e),
            _ => "An internal server error occurred".into(),
        }
    }

    pub fn response(&self) -> Response {
        let err_string = format!("Error: {}", &self);
        tracing::error!("{}", err_string);

        let status = self.status_code();
        let message = self.message();

        let body = serde_json::json!({
            "error": message,
        });

        (status, Json(body)).into_response()
    }
}

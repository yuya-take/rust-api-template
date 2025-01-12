use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Debug, Error)]
#[allow(clippy::enum_variant_names)]
pub enum ApiError {
    #[error("Database connection error")]
    DatabaseConnectionError(),

    #[error("Internal server error")]
    InternalServerError(String),

    #[error("Not found error")]
    NotFoundError(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::DatabaseConnectionError() => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database connection error".to_string(),
            ),
            ApiError::InternalServerError(msg) => {
                let message = format!("Internal server error: [{}]", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, message)
            }
            ApiError::NotFoundError(msg) => {
                let message = format!("Not found error: [{}]", msg);
                (StatusCode::NOT_FOUND, message)
            }
        }
        .into_response()
    }
}

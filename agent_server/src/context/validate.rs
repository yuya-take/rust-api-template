use async_trait::async_trait;
use axum::{
    extract::{rejection::JsonRejection, FromRequest, Json, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::de::DeserializeOwned;
use thiserror::Error;
use validator::Validate;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedForm<T>(pub T);

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AxumJsonRejection(#[from] JsonRejection),
}

#[async_trait]
impl<T, S> FromRequest<S> for ValidatedForm<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = ValidationError;

    async fn from_request(
        req: Request<axum::body::Body>,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        tracing::debug!("Validating form");
        let Json(value) = Json::<T>::from_request(req, state).await.map_err(|e| {
            tracing::error!("AxumJsonRejectionError: {:?}", e);
            ValidationError::AxumJsonRejection(e)
        })?;
        value.validate().map_err(|e| {
            tracing::error!("ValidationErrorError: {:?}", e);
            ValidationError::ValidationError(e)
        })?;
        Ok(ValidatedForm(value))
    }
}

impl IntoResponse for ValidationError {
    fn into_response(self) -> Response {
        match self {
            ValidationError::ValidationError(_) => {
                let message = format!("Input validation error: [{self}]").replace('\n', ", ");
                (StatusCode::BAD_REQUEST, message)
            }
            ValidationError::AxumJsonRejection(_) => (StatusCode::BAD_REQUEST, self.to_string()),
        }
        .into_response()
    }
}

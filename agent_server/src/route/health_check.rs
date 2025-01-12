use std::sync::Arc;

use axum::extract::Extension;
use axum::http::StatusCode;
use axum::response::IntoResponse;

use crate::context::error::ApiError;
use crate::module::{Modules, ModulesExt};

pub async fn hc_hello() -> impl IntoResponse {
    // レスポンスを返す
    tracing::info!("hc_hello");
    "Hello, World!"
}

pub async fn hc_postgres(
    Extension(module): Extension<Arc<Modules>>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::info!("hc_mysql");
    module
        .health_check_use_case()
        .diagnose_postgres_conn()
        .await
        .map(|_| (StatusCode::OK, "Hello Posgres!"))
        .map_err(|e| {
            tracing::error!("Failed to diagnose postgres conn: {:?}", e);
            ApiError::DatabaseConnectionError()
        })
}

pub async fn hc_dynamodb(
    Extension(module): Extension<Arc<Modules>>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::info!("hc_mysql");
    module
        .health_check_use_case()
        .diagnose_dynamodb_conn()
        .await
        .map(|_| (StatusCode::OK, "Hello DynamoDB!"))
        .map_err(|e| {
            tracing::error!("Failed to diagnose dynamodb conn: {:?}", e);
            ApiError::DatabaseConnectionError()
        })
}

//! Custom error types for the application

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

/// Our application error type
///
/// `thiserror::Error` derive macro auto-implements std::error::Error
/// and generates Display impl from the #[error("...")] attributes
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] diesel::result::Error),

    #[error("Connection pool error: {0}")]
    Pool(#[from] diesel::r2d2::PoolError),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

/// Tell Axum how to convert our errors into HTTP responses
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::Database(e) => {
                tracing::error!("Database error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error occurred")
            }
            AppError::Pool(e) => {
                tracing::error!("Pool error: {:?}", e);
                (StatusCode::SERVICE_UNAVAILABLE, "Service temporarily unavailable")
            }
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.as_str()),
            AppError::InvalidInput(msg) => (StatusCode::BAD_REQUEST, msg.as_str()),
            AppError::Internal(msg) => {
                tracing::error!("Internal error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
        };

        let body = Json(json!({
            "status": "error",
            "message": message,
        }));

        (status, body).into_response()
    }
}

/// Shorthand for Result<T, AppError>
pub type AppResult<T> = Result<T, AppError>;

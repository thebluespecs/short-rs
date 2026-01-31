//! Health check handler

use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    status: &'static str,
}

/// GET / - Health check endpoint
pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

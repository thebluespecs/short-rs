//! URL handlers - HTTP layer for URL shortening

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Redirect,
    Json,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::error::AppResult;
use crate::models::Url;
use crate::state::AppState;

// ============================================================================
// Request/Response types (DTOs)
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct CreateUrlRequest {
    pub url: String,
    pub expires_in_seconds: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct UrlResponse {
    pub original_url: String,
    pub short_code: String,
    pub short_url: String,
    pub visits: i64,
    pub expires_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}

impl From<Url> for UrlResponse {
    fn from(url: Url) -> Self {
        Self {
            short_url: format!("/{}", url.short_code),
            original_url: url.original_url,
            short_code: url.short_code,
            visits: url.visits,
            expires_at: url.expires_at,
            created_at: url.created_at,
        }
    }
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub status: &'static str,
    pub data: T,
}

// ============================================================================
// Handlers
// ============================================================================

/// POST /shorten - Create a shortened URL
///
/// #[tracing::instrument] automatically creates a span for this function.
/// - `skip(state)` = don't log the state (it's large/not useful)
/// - `fields(...)` = add custom fields to the span
#[tracing::instrument(
    name = "create_url",
    skip(state, payload),
    fields(
        original_url = %payload.url,
        expires_in = ?payload.expires_in_seconds
    )
)]
pub async fn create_url(
    State(state): State<AppState>,
    Json(payload): Json<CreateUrlRequest>,
) -> AppResult<(StatusCode, Json<ApiResponse<UrlResponse>>)> {
    tracing::info!("Creating shortened URL");

    let service = state.url_service;
    let original_url = payload.url.clone();

    let url = tokio::task::spawn_blocking(move || {
        service.create(&payload.url, payload.expires_in_seconds)
    })
    .await
    .map_err(|e| {
        tracing::error!(error = %e, "Task join error");
        crate::error::AppError::Internal(e.to_string())
    })??;

    tracing::info!(
        short_code = %url.short_code,
        id = url.id,
        "URL shortened successfully"
    );

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse {
            status: "ok",
            data: UrlResponse::from(url),
        }),
    ))
}

/// GET /:code/info - Get URL info without redirecting
#[tracing::instrument(
    name = "get_url_info",
    skip(state),
    fields(short_code = %code)
)]
pub async fn get_url_info(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> AppResult<Json<ApiResponse<UrlResponse>>> {
    tracing::debug!("Fetching URL info");

    let service = state.url_service;
    let url = tokio::task::spawn_blocking(move || service.get_by_code(&code))
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Task join error");
            crate::error::AppError::Internal(e.to_string())
        })??;

    tracing::debug!(
        original_url = %url.original_url,
        visits = url.visits,
        "URL info retrieved"
    );

    Ok(Json(ApiResponse {
        status: "ok",
        data: UrlResponse::from(url),
    }))
}

/// GET /:code - Redirect to original URL
#[tracing::instrument(
    name = "redirect",
    skip(state),
    fields(short_code = %code)
)]
pub async fn redirect(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> AppResult<Redirect> {
    tracing::debug!("Processing redirect");

    let service = state.url_service;
    let url = tokio::task::spawn_blocking(move || service.redirect(&code))
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Task join error");
            crate::error::AppError::Internal(e.to_string())
        })??;

    tracing::info!(
        original_url = %url.original_url,
        visits = url.visits,
        "Redirecting"
    );

    Ok(Redirect::temporary(&url.original_url))
}

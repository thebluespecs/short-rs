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
// Request/Response types (DTOs) - live here, close to the HTTP layer
// ============================================================================

/// Request body for POST /shorten
#[derive(Debug, Deserialize)]
pub struct CreateUrlRequest {
    pub url: String,
    pub expires_in_seconds: Option<i64>,
}

/// Response for URL operations
#[derive(Debug, Serialize)]
pub struct UrlResponse {
    pub original_url: String,
    pub short_code: String,
    pub short_url: String,
    pub visits: i64,
    pub expires_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}

/// Convert database model to API response
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

/// Generic API response wrapper
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
/// `State<T>` is Axum's way of sharing data across handlers (like context in Go).
/// We extract UrlService from the app state.
///
/// `Json<T>` extracts and deserializes the request body.
pub async fn create_url(
    State(state): State<AppState>,
    Json(payload): Json<CreateUrlRequest>,
) -> AppResult<(StatusCode, Json<ApiResponse<UrlResponse>>)> {
    let service = state.url_service;
    let url = tokio::task::spawn_blocking(move || {
        service.create(&payload.url, payload.expires_in_seconds)
    })
    .await
    .map_err(|e| crate::error::AppError::Internal(e.to_string()))??;
    // ^^ Two `?`: first for JoinError, second for AppError

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse {
            status: "ok",
            data: UrlResponse::from(url),
        }),
    ))
}

/// GET /:code/info - Get URL info without redirecting
pub async fn get_url_info(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> AppResult<Json<ApiResponse<UrlResponse>>> {
    let service = state.url_service;
    let url = tokio::task::spawn_blocking(move || service.get_by_code(&code))
        .await
        .map_err(|e| crate::error::AppError::Internal(e.to_string()))??;

    Ok(Json(ApiResponse {
        status: "ok",
        data: UrlResponse::from(url),
    }))
}

/// GET /:code - Redirect to original URL
///
/// Returns a `Redirect` which Axum turns into a 307 redirect response.
pub async fn redirect(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> AppResult<Redirect> {
    let service = state.url_service;
    let url = tokio::task::spawn_blocking(move || service.redirect(&code))
        .await
        .map_err(|e| crate::error::AppError::Internal(e.to_string()))??;

    // Redirect::temporary = 307, keeps the original request method
    // Redirect::permanent = 301, may be cached by browsers
    Ok(Redirect::temporary(&url.original_url))
}

//! Routes module - defines all API routes

use axum::{
    routing::{get, post},
    Router,
};

use crate::handlers;
use crate::state::AppState;

/// Create the application router with all routes
pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(handlers::health_check))
        .route("/shorten", post(handlers::create_url))
        .route("/:code/info", get(handlers::get_url_info))
        .route("/:code", get(handlers::redirect))
        .with_state(state)
}

//! Short-It: URL Shortener Service

mod config;
mod db;
mod error;
mod handlers;
mod models;
mod routes;
mod services;
mod state;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::Config;
use crate::state::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "short_it=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env()?;
    tracing::info!("Starting server on {}", config.address());

    // Build app state (all services wired up inside)
    let pool = db::create_pool(&config.database_url)?;
    let state = AppState::new(pool);

    // Create router
    let app = routes::create_router(state);

    // Start server
    let listener = tokio::net::TcpListener::bind(&config.address()).await?;
    tracing::info!("Listening on http://{}", config.address());

    axum::serve(listener, app).await?;

    Ok(())
}

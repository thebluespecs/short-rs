//! Application state - bundles all services for dependency injection

use crate::db::repositories::UrlRepository;
use crate::db::DbPool;
use crate::services::UrlService;

/// Holds all application services
///
/// This is what gets passed to handlers via `State<AppState>`.
/// Clone is cheap because services internally use Arc (reference counting).
#[derive(Clone)]
pub struct AppState {
    pub url_service: UrlService,
    // Add more services here as the app grows:
    // pub user_service: UserService,
    // pub auth_service: AuthService,
}

impl AppState {
    /// Build all services from a database pool
    pub fn new(pool: DbPool) -> Self {
        // Build repositories
        let url_repo = UrlRepository::new(pool);

        // Build services (inject repos and other services as needed)
        let url_service = UrlService::new(url_repo);

        Self { url_service }
    }
}

//! URL Service - business logic for URL shortening

use chrono::{Duration, Utc};

use crate::db::repositories::UrlRepository;
use crate::error::{AppError, AppResult};
use crate::models::{NewUrl, Url};
use crate::services::base62;

/// Service layer for URL operations
///
/// Sits between handlers (HTTP) and repository (database).
/// Contains business logic like encoding IDs, validating URLs, etc.
#[derive(Clone)]
pub struct UrlService {
    repo: UrlRepository,
}

impl UrlService {
    pub fn new(repo: UrlRepository) -> Self {
        Self { repo }
    }

    /// Create a shortened URL
    pub fn create(&self, original_url: &str, expires_in_seconds: Option<i64>) -> AppResult<Url> {
        // Basic URL validation
        if !original_url.starts_with("http://") && !original_url.starts_with("https://") {
            return Err(AppError::InvalidInput(
                "URL must start with http:// or https://".to_string(),
            ));
        }

        // Calculate expiration time if provided
        let expires_at = expires_in_seconds.map(|secs| {
            (Utc::now() + Duration::seconds(secs)).naive_utc()
        });

        // Generate short code from timestamp + random factor for uniqueness
        // We'll update this to use the ID after insert
        let temp_code = format!("temp_{}", Utc::now().timestamp_nanos_opt().unwrap_or(0));

        let new_url = NewUrl {
            original_url,
            short_code: &temp_code,
            expires_at,
        };

        let created = self.repo.create(new_url)?;

        // Now update with the real short code based on ID
        let short_code = base62::encode(created.id);
        self.repo.update_short_code(created.id, &short_code)
    }

    /// Get URL info by short code
    pub fn get_by_code(&self, short_code: &str) -> AppResult<Url> {
        self.repo.find_by_code(short_code)
    }

    /// Redirect: get URL and increment visits
    pub fn redirect(&self, short_code: &str) -> AppResult<Url> {
        let url = self.repo.find_by_code(short_code)?;

        // Check if expired
        if let Some(expires_at) = url.expires_at {
            if expires_at < Utc::now().naive_utc() {
                return Err(AppError::NotFound("URL has expired".to_string()));
            }
        }

        // Increment visits and return
        self.repo.increment_visits(url.id)
    }
}

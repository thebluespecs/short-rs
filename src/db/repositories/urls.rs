//! URL Repository - database operations for urls table

use diesel::prelude::*;

use crate::db::schema::urls;
use crate::db::DbPool;
use crate::error::{AppError, AppResult};
use crate::models::{NewUrl, Url};

#[derive(Clone)]
pub struct UrlRepository {
    pool: DbPool,
}

impl UrlRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub fn create(&self, new_url: NewUrl) -> AppResult<Url> {
        let mut conn = self.pool.get()?;

        diesel::insert_into(urls::table)
            .values(&new_url)
            .returning(Url::as_returning())
            .get_result(&mut conn)
            .map_err(AppError::from)
    }

    pub fn find_by_code(&self, code: &str) -> AppResult<Url> {
        let mut conn = self.pool.get()?;

        urls::table
            .filter(urls::short_code.eq(code))
            .select(Url::as_select())
            .first(&mut conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => {
                    AppError::NotFound(format!("URL with code '{}' not found", code))
                }
                other => AppError::from(other),
            })
    }

    pub fn _find_by_id(&self, id: i64) -> AppResult<Url> {
        let mut conn = self.pool.get()?;

        urls::table
            .find(id)
            .select(Url::as_select())
            .first(&mut conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => {
                    AppError::NotFound(format!("URL with id '{}' not found", id))
                }
                other => AppError::from(other),
            })
    }

    pub fn increment_visits(&self, id: i64) -> AppResult<Url> {
        let mut conn = self.pool.get()?;

        diesel::update(urls::table.find(id))
            .set(urls::visits.eq(urls::visits + 1))
            .returning(Url::as_returning())
            .get_result(&mut conn)
            .map_err(AppError::from)
    }

    /// Update the short_code for a URL (used after generating code from ID)
    pub fn update_short_code(&self, id: i64, short_code: &str) -> AppResult<Url> {
        let mut conn = self.pool.get()?;

        diesel::update(urls::table.find(id))
            .set(urls::short_code.eq(short_code))
            .returning(Url::as_returning())
            .get_result(&mut conn)
            .map_err(AppError::from)
    }
}

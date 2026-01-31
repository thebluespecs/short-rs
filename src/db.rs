//! Database module - connection pool and submodules
//!
//! Structure:
//!   src/db.rs              <- You are here (module entry)
//!   src/db/schema.rs       <- All table definitions
//!   src/db/repositories/   <- Database operations (one per entity)

pub mod schema;
pub mod repositories;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn create_pool(database_url: &str) -> Result<DbPool, String> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .max_size(10)
        .build(manager)
        .map_err(|e| format!("Failed to create pool: {}", e))
}

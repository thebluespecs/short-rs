//! Database models - data structures for DB operations

use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;

use crate::db::schema::urls;

/// A URL record from the database (for SELECT queries)
#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Serialize)]
#[diesel(table_name = urls)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Url {
    pub id: i64,
    pub original_url: String,
    pub short_code: String,
    pub visits: i64,
    pub expires_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// For INSERT operations
#[derive(Debug, Insertable)]
#[diesel(table_name = urls)]
pub struct NewUrl<'a> {
    pub original_url: &'a str,
    pub short_code: &'a str,
    pub expires_at: Option<NaiveDateTime>,
}

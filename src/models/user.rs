use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct User {
    id: i32,
    name: String,
    email: String,
    password: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    deleted_at: Option<DateTime<Utc>>,
}

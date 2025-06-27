use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub created_on_utc: DateTime<Utc>,
}
use anyhow::{Error, Ok, Result, anyhow};

use async_trait::async_trait;
use chrono::Utc;
use sqlx::PgPool;
use std::sync::Arc;

use crate::{
    entities::user::User, models::users::user_create_model::UserCreateModel,
    repository::contracts::user_repository::UserRepository,
};

pub struct PgUserRepository {
    pub pool: Arc<PgPool>,
}

impl PgUserRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn save(&self, user: &UserCreateModel) -> Result<bool, Error> {
        let now = Utc::now();
        let result = sqlx::query(
            "INSERT INTO users(username, password_hash, created_on_utc) VALUES ($1, $2, $3)",
        )
        .bind(&user.username)
        .bind(&user.password_hash)
        .bind(now)
        .execute(&*self.pool)
        .await
        .map_err(|e| anyhow!(e))?;

        let rows_affected = result.rows_affected();
        Ok(rows_affected > 0)
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, Error> {
        let row = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, password_hash, created_on_utc
            FROM users
            WHERE username = $1
            "#,
            username
        )
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| anyhow!(e))?;

        Ok(row)
    }
}

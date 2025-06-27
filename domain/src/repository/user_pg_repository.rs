use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;
use chrono::Utc;

use crate::{
    entities::user::User,
    models::users::user_create_model::UserCreateModel,
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
    async fn save(&self, user: &UserCreateModel) {
        let now = Utc::now();
        let _ = sqlx::query("INSERT INTO users(username, password_hash, created_on_utc) VALUES ($1, $2, $3)")
            .bind(&user.username)
            .bind(&user.password_hash)
            .bind(now)
            .execute(&*self.pool)
            .await;
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, sqlx::Error> {
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
        .await;

        row
    }
}

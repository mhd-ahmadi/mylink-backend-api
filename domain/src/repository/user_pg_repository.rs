use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;

use crate::{
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
        let _ = sqlx::query("INSERT INTO users(username, password_hash2) VALUES ($1, $2)")
            .bind(&user.username)
            .bind(&user.password_hash)
            .execute(&*self.pool)
            .await;
    }
}

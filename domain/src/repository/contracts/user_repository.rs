use async_trait::async_trait;

use crate::{entities::user::User, models::users::user_create_model::UserCreateModel};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn save(&self, user: &UserCreateModel);

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, sqlx::Error>;
}

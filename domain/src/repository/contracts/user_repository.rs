use async_trait::async_trait;
use anyhow::{Result, Error};

use crate::models::users::user_create_model::UserCreateModel;

#[async_trait]
pub trait UserRepository {
    async fn save(&self, user: &UserCreateModel)-> Result<(), Error> ;
}

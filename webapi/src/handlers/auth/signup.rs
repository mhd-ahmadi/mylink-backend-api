use std::sync::Arc;

use crate::{
    base::{
        api_response::{ApiHttpResponse, ApiResponse},
        app_state::AppState,
    },
    handlers::auth::sign_up_request::SignUpRequest,
};
use axum::{Extension, Json};

use axum_valid::Valid;
use domain::repository::user_pg_repository::PgUserRepository;
use domain::{
    models::users::user_create_model::UserCreateModel,
    repository::contracts::user_repository::UserRepository,
};
use serde_json::json;
use utils::password_hasher::PasswordHasher;

#[utoipa::path(post, path = "/auth/signup", tag = "Auth")]
pub async fn signup(
    Extension(state): Extension<Arc<AppState>>,
    request: Valid<Json<SignUpRequest>>,
) -> ApiResponse {

    let hasher= PasswordHasher::new();
    let hash=hasher.hash_password(&request.password).expect("Failed to hash");

    let user_repo = PgUserRepository::new(Arc::new(state.db.clone()));

    let user_model = UserCreateModel {
        password_hash: hash,
        username: request.username.clone(),
    };

    let result = user_repo.save(&user_model).await;

    ApiHttpResponse::success(Some(json!(result)))
        .with_success_message(None)
        .build()
}

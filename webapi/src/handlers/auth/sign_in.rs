use std::sync::Arc;

use crate::{
    base::{
        api_response::{ApiHttpResponse, ApiResponse},
        app_state::AppState,
    },
    handlers::auth::sign_in_request::SignInRequest,
};
use axum::{Extension, Json};
use axum_valid::Valid;
use domain::{
    repository::{
        contracts::user_repository::UserRepository,
        user_pg_repository::PgUserRepository,
    },
};
use serde_json::json;
use utils::password_hasher::PasswordHasher;

#[utoipa::path(post, path = "/auth/sign_in", tag = "Auth")]
pub async fn sign_in(
    Extension(state): Extension<Arc<AppState>>,
    request: Valid<Json<SignInRequest>>,
) -> ApiResponse {
    let user_repo = PgUserRepository::new(Arc::new(state.db.clone()));
    match user_repo.find_by_username(&request.username).await {
        Ok(Some(user)) => {
            let hasher = PasswordHasher::new();
            let is_valid = hasher.verify_password(&request.password, &user.password_hash);

            if is_valid {
                ApiHttpResponse::success(Some(json!({
                    "message": "Authentication successful"
                })))
                .build()
            } else {
                ApiHttpResponse::error()
                    .with_error_message(Some("Invalid username or password"))
                    .build()
            }
        }
        Ok(None) => ApiHttpResponse::error()
            .with_error_message(Some("User not found"))
            .build(),
        Err(e) => ApiHttpResponse::error()
            .with_error_message(Some("Database error"))
            .build(),
    }
}

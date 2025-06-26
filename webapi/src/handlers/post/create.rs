use crate::{
    base::{
        api_response::{ApiHttpResponse, ApiResponse},
        app_state::AppState,
    },
    extractors::auth::AuthUser,
};
use axum::Extension;
use serde_json::json;
use std::sync::Arc;

#[utoipa::path(post, path = "/posts", tag = "Posts")]
pub async fn create_post(
    AuthUser(user): AuthUser,
    Extension(state): Extension<Arc<AppState>>,
) -> ApiResponse {
    ApiHttpResponse::success(Some(json!("create_post_result"))).build()
}

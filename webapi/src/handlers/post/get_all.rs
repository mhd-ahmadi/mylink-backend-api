use std::sync::Arc;
use axum::{Extension};
use serde_json::json;

use crate::{
    base::{
        api_response::{ApiHttpResponse, ApiResponse},
        app_state::AppState,
    },
    extractors::auth::AuthUser,
};

#[utoipa::path(get, path = "/posts", tag = "Posts")]
pub async fn get_all_posts(
    AuthUser(user): AuthUser,
    Extension(state): Extension<Arc<AppState>>,
) -> ApiResponse {
    println!("{}", state.configuration.database_url);
    ApiHttpResponse::success(Some(json!("get_all_result"))).build()
}

use crate::handlers::post::create::create_post;
use crate::handlers::post::get_all::get_all_posts;

use axum::routing::{get, post};
use utoipa_axum::{router::OpenApiRouter};

pub fn router() -> OpenApiRouter {
    return OpenApiRouter::new()
        .route("/", post(create_post))
        .route("/", get(get_all_posts));
}

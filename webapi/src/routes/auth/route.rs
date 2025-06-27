use axum::routing::post;
use utoipa_axum::router::OpenApiRouter;

use crate::handlers::auth::{sign_in::sign_in, sign_up::sign_up};

pub fn router() -> OpenApiRouter {
    return OpenApiRouter::new()
        .route("/sign_up", post(sign_up))
        .route("/sign_in", post(sign_in));
}

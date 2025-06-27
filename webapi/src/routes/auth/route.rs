use axum::routing::{post};
use utoipa_axum::{router::OpenApiRouter};

use crate::handlers::auth::sign_up::sign_up;


pub fn router() -> OpenApiRouter {
    return OpenApiRouter::new()
        .route("/signup", post(sign_up));
}

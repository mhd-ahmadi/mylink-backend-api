use axum::routing::{post};
use utoipa_axum::{router::OpenApiRouter};

use crate::handlers::auth::signup::signup;


pub fn router() -> OpenApiRouter {
    return OpenApiRouter::new()
        .route("/signup", post(signup));
}

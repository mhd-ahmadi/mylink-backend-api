mod base;
mod extractors;
mod handlers;
mod messages;
mod routes;

use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc
};

use axum::{
    Extension, Router, http::StatusCode,
    middleware::from_extractor,routing::get, serve,
};

use base::configuration::Configuration;
use base::cors_config::cors_config;
use base::pg_connection::create_pg_connection;
use sqlx::{Pool, Postgres};
use tower::{BoxError};
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

use crate::{base::app_state::AppState, extractors::auth::AuthUser};
use base::api_response::{ApiHttpResponse, ApiResponse};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

use routes::api_docs::ApiDoc;
use std::io::Error;
use utoipa::OpenApi;

async fn handle_error(err: BoxError) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("خطای داخلی: {}", err),
    )
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let configuration: Configuration = Configuration::init();
    let db_pool: Pool<Postgres> = create_pg_connection(&configuration.database_url).await;

    let cors: CorsLayer = cors_config();

    let app_state: Arc<AppState> = Arc::new(AppState {
        db: db_pool,
        configuration,
    });

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .route("/", get(root_handler))
        .nest("/auth", routes::auth::route::router())
        .nest(
            "/posts",
            routes::post::route::router().route_layer(from_extractor::<AuthUser>()),
        )
        .layer(Extension(app_state))
        .layer(cors)
        .split_for_parts();

    let router: Router =
        router.merge(SwaggerUi::new("/swagger").url("/docs/openapi.json", api.clone()));

    let address: SocketAddr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 3000));
    let listener: TcpListener = TcpListener::bind(&address).await?;
    serve(listener, router.into_make_service()).await
}

async fn root_handler(Extension(_state): Extension<Arc<AppState>>) -> ApiResponse {
    println!("{}", _state.configuration.database_url);
    ApiHttpResponse::success(None).build()
}

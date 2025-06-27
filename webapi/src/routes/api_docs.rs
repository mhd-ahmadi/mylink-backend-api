use utoipa::{
    Modify, OpenApi,
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
};

#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon),
    paths(
        crate::handlers::auth::sign_in::sign_in,
        crate::handlers::auth::sign_up::sign_up,

        crate::handlers::post::create::create_post,
        crate::handlers::post::get_all::get_all_posts,
    ),
    tags(
        (name = "Auth"),
        (name= "Posts")
    )
)]
pub struct ApiDoc;

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("My Link"))),
            )
        }
    }
}

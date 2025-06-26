use std::sync::Arc;

use crate::base::{
    api_response::{ApiHttpResponse, ApiResponse},
    app_state::AppState,
    user_claims::Claims,
};
use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use futures::future::BoxFuture;
use jsonwebtoken::{Algorithm, DecodingKey, TokenData, Validation, decode};

pub struct AuthUser(pub Claims);

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, ApiResponse);

    #[allow(refining_impl_trait)]
    fn from_request_parts<'a, 'b>(
        parts: &'a mut Parts,
        _state: &'b S,
    ) -> BoxFuture<'a, Result<Self, Self::Rejection>> {
        Box::pin(async move {
            let app_state: &Arc<AppState> = parts.extensions.get::<Arc<AppState>>().ok_or((
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiHttpResponse::error().with_error_message(None).build(),
            ))?;

            let auth_header: Option<&str> = parts
                .headers
                .get("Authorization")
                .and_then(|v| v.to_str().ok());

            let token: &str = match auth_header {
                Some(h) if h.starts_with("Bearer ") => &h[7..],
                Some(_) => {
                    return Err((
                        StatusCode::UNAUTHORIZED,
                        ApiHttpResponse::error().with_error_message(None).build(),
                    ));
                }
                None => {
                    return Err((
                        StatusCode::UNAUTHORIZED,
                        ApiHttpResponse::error().with_error_message(None).build(),
                    ));
                }
            };

            let token_data: Result<TokenData<Claims>, _> = decode::<Claims>(
                token,
                &DecodingKey::from_secret(&app_state.configuration.jwt_secret.as_bytes()),
                &Validation::new(Algorithm::HS256),
            );

            match token_data {
                Ok(data) => Ok(AuthUser(data.claims)),
                Err(_) => Err((
                    StatusCode::UNAUTHORIZED,
                    ApiHttpResponse::error().with_error_message(None).build(),
                )),
            }
        })
    }
}

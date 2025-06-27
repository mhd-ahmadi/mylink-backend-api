use crate::CONFIG;

pub struct JwtManager;

impl JwtManager {
    pub fn generate_token(username: &str) -> String {
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("valid timestamp")
            .timestamp();

        let claims = Claims {
            sub: username.to_owned(),
            exp: expiration as usize,
        };

        let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET not set");

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .expect("Token creation failed")
    }
}

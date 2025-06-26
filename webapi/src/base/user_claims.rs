use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub role_id: i32,
    pub email: String,
    pub exp: usize,
}

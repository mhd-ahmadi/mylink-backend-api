use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Validate,ToSchema)]
pub struct SignUpRequest {
    #[validate(length(min = 3, message = "نام باید حداقل ۳ حرف باشد"))]
    pub username: String,

    #[validate(length(min = 3, message = "نام باید حداقل ۳ حرف باشد"))]
    pub password: String,
}

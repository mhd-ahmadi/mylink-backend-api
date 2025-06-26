use serde::Serialize;

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::messages::common::error::OPERATION_WAS_FAILED;
use crate::messages::common::success::OPERATION_WAS_SUCCESSFUL;

use serde;

#[derive(Serialize)]
pub struct ApiHttpResponse<T>
where
    T: Serialize,
{
    pub data: Option<T>,
    pub message: Option<String>,
    pub messages: Option<Vec<String>>,
    pub exception: Option<serde_json::Value>,
}

pub struct ApiHttpResponseBuilder<T>
where
    T: Serialize,
{
    data: Option<T>,
    message: Option<String>,
    messages: Option<Vec<String>>,
    exception: Option<serde_json::Value>,
}

impl<T> ApiHttpResponseBuilder<T>
where
    T: Serialize,
{
    pub fn with_success_message(mut self, msg: Option<&str>) -> Self {
        self.message = Some(msg.unwrap_or(OPERATION_WAS_SUCCESSFUL).to_string());
        self
    }

    pub fn with_error_message(mut self, msg: Option<&str>) -> Self {
        self.message = Some(msg.unwrap_or(OPERATION_WAS_FAILED).to_string());
        self
    }

    pub fn with_messages(mut self, msgs: Vec<String>) -> Self {
        self.messages = Some(msgs);
        self
    }

    pub fn with_exception(mut self, exp: serde_json::Value) -> Self {
        self.exception = Some(exp);
        self
    }

    pub fn build(self) -> ApiHttpResponse<T> {
        ApiHttpResponse {
            data: self.data,
            message: self.message,
            messages: self.messages,
            exception: self.exception,
        }
    }
}

impl<T> ApiHttpResponse<T>
where
    T: Serialize,
{
    pub fn success(data: Option<T>) -> ApiHttpResponseBuilder<T> {
        ApiHttpResponseBuilder {
            data: data,
            message: None,
            messages: None,
            exception: None,
        }
    }

    pub fn error() -> ApiHttpResponseBuilder<T> {
        ApiHttpResponseBuilder {
            data: None,
            message: None,
            messages: None,
            exception: None,
        }
    }
}

impl<T> IntoResponse for ApiHttpResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let status = if self.data.is_some() {
            StatusCode::OK
        } else {
            StatusCode::INTERNAL_SERVER_ERROR
        };

        (status, Json(self)).into_response()
    }
}

pub type ApiResponse = ApiHttpResponse<serde_json::Value>;

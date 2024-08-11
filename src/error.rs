use std::{error, fmt};

use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use utoipa::{ToResponse, ToSchema};

#[derive(Debug, Clone, ToResponse, ToSchema)]
#[response(description = "Error in processing the request")]
pub struct AppError {
    #[schema(value_type=i32)]
    error_code: StatusCode,
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response<Body> {
        Response::builder()
            .status(self.error_code)
            .body(Body::new(self.message))
            .unwrap()
    }
}

impl AppError {
    pub fn internal_error<T: Into<String>>(message: T) -> Self {
        Self {
            error_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: message.into(),
        }
    }
}

impl From<anyhow::Error> for AppError {
    fn from(value: anyhow::Error) -> Self {
        Self {
            error_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: value.to_string(),
        }
    }
}

impl error::Error for AppError {}
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.error_code, self.message.as_str())
    }
}

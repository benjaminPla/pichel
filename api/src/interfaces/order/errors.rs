use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use crate::application::order::errors::OrderAppError;

#[derive(Debug)]
pub struct OrderInterError(StatusCode, String);

impl From<OrderAppError> for OrderInterError {
    fn from(e: OrderAppError) -> Self {
        match e {
            OrderAppError::Internal(msg)        => {
                tracing::error!(error = %msg);
                Self(StatusCode::INTERNAL_SERVER_ERROR, "internal server error".to_string())
            }
            OrderAppError::NotFound              => Self(StatusCode::NOT_FOUND, "not found".to_string()),
            OrderAppError::Validation(msg)       => Self(StatusCode::BAD_REQUEST, msg),
        }
    }
}

impl IntoResponse for OrderInterError {
    fn into_response(self) -> Response {
        (self.0, Json(json!({ "error": self.1 }))).into_response()
    }
}

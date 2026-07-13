use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use crate::application::category::errors::CategoryAppError;

#[derive(Debug)]
pub struct CategoryInterError(pub StatusCode, pub String);

impl From<CategoryAppError> for CategoryInterError {
    fn from(e: CategoryAppError) -> Self {
        match e {
            CategoryAppError::Internal(msg) => {
                tracing::error!(error = %msg);
                Self(StatusCode::INTERNAL_SERVER_ERROR, "internal server error".to_string())
            }
            CategoryAppError::NotFound        => Self(StatusCode::NOT_FOUND, "not found".to_string()),
            CategoryAppError::Validation(msg) => Self(StatusCode::BAD_REQUEST, msg),
        }
    }
}

impl IntoResponse for CategoryInterError {
    fn into_response(self) -> Response {
        (self.0, Json(json!({ "error": self.1 }))).into_response()
    }
}

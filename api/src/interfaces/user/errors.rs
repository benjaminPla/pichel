use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use crate::application::user::errors::UserAppError;

#[derive(Debug)]
pub struct UserInterError(StatusCode, String);

impl From<UserAppError> for UserInterError {
    fn from(e: UserAppError) -> Self {
        match e {
            UserAppError::Internal        => Self(StatusCode::INTERNAL_SERVER_ERROR, "internal server error".to_string()),
            UserAppError::NotFound        => Self(StatusCode::NOT_FOUND, "not found".to_string()),
            UserAppError::Validation(msg) => Self(StatusCode::BAD_REQUEST, msg.to_string()),
        }
    }
}

impl IntoResponse for UserInterError {
    fn into_response(self) -> Response {
        (self.0, Json(json!({ "error": self.1 }))).into_response()
    }
}

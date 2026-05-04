use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum AuthInterError {
    #[error("token expired")]
    ExpiredToken,
    #[error("invalid token")]
    InvalidToken,
    #[error("missing token")]
    MissingToken,
}

impl IntoResponse for AuthInterError {
    fn into_response(self) -> Response {
        let (status, msg) = match self {
            Self::ExpiredToken => (StatusCode::UNAUTHORIZED, self.to_string()),
            Self::InvalidToken => (StatusCode::UNAUTHORIZED, self.to_string()),
            Self::MissingToken => (StatusCode::UNAUTHORIZED, self.to_string()),
        };
        (status, Json(json!({ "error": msg }))).into_response()
    }
}

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use crate::application::auth::errors::AuthAppError;
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum AuthInterError {
    #[error("internal server error")]
    Internal(String),
    #[error("invalid token")]
    InvalidToken,
    #[error("missing token")]
    MissingToken,
    #[error("unauthorized")]
    Unauthorized,
}

impl From<AuthAppError> for AuthInterError {
    fn from(e: AuthAppError) -> Self {
        match e {
            AuthAppError::Internal(msg) => { tracing::error!("{msg}"); Self::Internal(msg) },
            AuthAppError::Unauthorized  => Self::Unauthorized,
        }
    }
}

impl IntoResponse for AuthInterError {
    fn into_response(self) -> Response {
        let (status, msg) = match &self {
            Self::Internal(msg)        => {
                tracing::error!(error = %msg);
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            Self::InvalidToken    => (StatusCode::UNAUTHORIZED, self.to_string()),
            Self::MissingToken    => (StatusCode::UNAUTHORIZED, self.to_string()),
            Self::Unauthorized    => (StatusCode::UNAUTHORIZED, self.to_string()),
        };
        (status, Json(json!({ "error": msg }))).into_response()
    }
}

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use crate::{application::user::errors::UserAppError, domain::user::repository::UserRepoError};

#[derive(Debug)]
pub struct UserInterError(StatusCode, String);

impl From<UserAppError> for UserInterError {
    fn from(err: UserAppError) -> Self {
        match err {
            UserAppError::Email(e)                        => Self(StatusCode::UNPROCESSABLE_ENTITY, e.to_string()),
            UserAppError::PasswordRaw(e)                  => Self(StatusCode::UNPROCESSABLE_ENTITY, e.to_string()),
            UserAppError::Repo(UserRepoError::Database)   => Self(StatusCode::INTERNAL_SERVER_ERROR, "internal server error".into()),
            UserAppError::Repo(UserRepoError::Mapping(m)) => { Self(StatusCode::INTERNAL_SERVER_ERROR, m) }
            UserAppError::UserHasherError(e)              => { Self(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()) }
        }
    }
}

impl IntoResponse for UserInterError {
    fn into_response(self) -> Response {
        (self.0, Json(json!({ "error": self.1 }))).into_response()
    }
}

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use crate::domain::{auth::AuthDomainError, product::ProductDomainError};

#[derive(Debug)]
pub enum AppError {
    Unauthorized,
    BadRequest(String),
    NotFound,
    Internal,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Unauthorized  => (StatusCode::UNAUTHORIZED,            "unauthorized".to_string()),
            AppError::BadRequest(m) => (StatusCode::BAD_REQUEST,             m),
            AppError::NotFound      => (StatusCode::NOT_FOUND,               "not found".to_string()),
            AppError::Internal      => (StatusCode::INTERNAL_SERVER_ERROR,   "internal server error".to_string()),
        };
        (status, Json(json!({ "error": message }))).into_response()
    }
}

impl From<AuthDomainError> for AppError {
    fn from(e: AuthDomainError) -> Self {
        match e {
            AuthDomainError::InvalidEmail
            | AuthDomainError::EmailAlreadyTaken => AppError::BadRequest(e.to_string()),
            AuthDomainError::InvalidPassword
            | AuthDomainError::Token(_)          => AppError::Unauthorized,
            AuthDomainError::HashingFailed
            | AuthDomainError::Internal          => AppError::Internal,
        }
    }
}

impl From<ProductDomainError> for AppError {
    fn from(e: ProductDomainError) -> Self {
        match e {
            ProductDomainError::EmptyName
            | ProductDomainError::NegativePrice => AppError::BadRequest(e.to_string()),
            ProductDomainError::NotFound        => AppError::NotFound,
            ProductDomainError::Internal        => AppError::Internal,
        }
    }
}

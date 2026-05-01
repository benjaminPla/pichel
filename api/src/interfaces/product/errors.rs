use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use crate::application::product::errors::ProductAppError;

#[derive(Debug)]
pub struct ProductInterError(StatusCode, String);

impl From<ProductAppError> for ProductInterError {
    fn from(e: ProductAppError) -> Self {
        match e {
            ProductAppError::Internal(msg)   => { tracing::error!(error = %msg); Self(StatusCode::INTERNAL_SERVER_ERROR, "internal server error".to_string()) }
            ProductAppError::NotFound        => { tracing::warn!("product not found");           Self(StatusCode::NOT_FOUND, "not found".to_string()) }
            ProductAppError::Validation(msg) => { tracing::warn!(error = %msg, "validation");    Self(StatusCode::BAD_REQUEST, msg) }
        }
    }
}

impl IntoResponse for ProductInterError {
    fn into_response(self) -> Response {
        (self.0, Json(json!({ "error": self.1 }))).into_response()
    }
}

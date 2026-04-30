use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use crate::{
    application::product::errors::ProductAppError, domain::product::ports::repository::ProductRepoError,
};

#[derive(Debug)]
pub struct ProductInterError(StatusCode, String);

impl From<ProductAppError> for ProductInterError {
    fn from(err: ProductAppError) -> Self {
        match err {
            ProductAppError::Description(e)                     => Self(StatusCode::UNPROCESSABLE_ENTITY, e.to_string()),
            ProductAppError::Name(e)                            => Self(StatusCode::UNPROCESSABLE_ENTITY, e.to_string()),
            ProductAppError::Repo(ProductRepoError::Database)   => Self(StatusCode::INTERNAL_SERVER_ERROR, "internal server error".into()),
            ProductAppError::Repo(ProductRepoError::Mapping(m)) => Self(StatusCode::INTERNAL_SERVER_ERROR, m),
            ProductAppError::Symbol(e)                          => Self(StatusCode::UNPROCESSABLE_ENTITY, e.to_string()),
            ProductAppError::UnitOfMeasure(e)                   => Self(StatusCode::UNPROCESSABLE_ENTITY, e.to_string()),
        }
    }
}

impl IntoResponse for ProductInterError {
    fn into_response(self) -> Response {
        (self.0, Json(json!({ "error": self.1 }))).into_response()
    }
}

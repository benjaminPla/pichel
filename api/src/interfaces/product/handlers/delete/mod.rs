pub mod dto;

use axum::{extract::{Path,State}, http::StatusCode, response::IntoResponse, Json};
use crate::{
    application::product::delete::{DeleteProductInput, DeleteProductUseCase},
    interfaces::{
        app_state::AppState,
        product::{
            errors::ProductInterError,
            handlers::delete::dto::ProductDeleteResponse,
        },
    },
};
use uuid::Uuid;

pub async fn delete(
    State(app_state): State<AppState>,
    Path(id):         Path<Uuid>,
) -> Result<impl IntoResponse, ProductInterError> {
    let product = DeleteProductUseCase::new(app_state.product_repo)
        .execute(DeleteProductInput { id })
        .await?;
    Ok((StatusCode::GONE, Json(ProductDeleteResponse::from(product))))
}

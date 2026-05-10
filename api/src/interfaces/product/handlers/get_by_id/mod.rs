pub mod dto;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;
use crate::{
    application::product::get_by_id::{GetProductByIdInput, GetProductByIdUseCase},
    interfaces::{
        app_state::AppState,
        product::{
            errors::ProductInterError,
            handlers::get_by_id::dto::ProductGetByIdResponse,
        },
    },
};

pub async fn get_by_id(
    State(app_state): State<AppState>,
    Path(id):         Path<Uuid>,
) -> Result<impl IntoResponse, ProductInterError> {
    let product = GetProductByIdUseCase::new(app_state.product_repo)
        .execute(GetProductByIdInput { id })
        .await?;
    Ok((StatusCode::OK, Json(ProductGetByIdResponse::from(product))))
}

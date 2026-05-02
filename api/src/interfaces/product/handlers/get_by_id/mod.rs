pub mod dto;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;
use crate::{
    application::product::queries::get_by_id::{ProductGetByIdHandler, ProductGetByIdQuery},
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
    let product = ProductGetByIdHandler::new(app_state.product_repo)
        .execute(ProductGetByIdQuery { id })
        .await?;
    Ok((StatusCode::OK, Json(ProductGetByIdResponse::from(product))))
}

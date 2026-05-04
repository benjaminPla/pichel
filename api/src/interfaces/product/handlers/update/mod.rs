pub mod dto;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use crate::{
    application::product::commands::update::{ProductUpdateCommand, ProductUpdateHandler},
    interfaces::{
        app_state::AppState,
        product::{
            errors::ProductInterError,
            handlers::update::dto::{ProductUpdateRequestBody, ProductUpdateResponse},
        },
    },
};
use uuid::Uuid;

pub async fn update(
    State(app_state): State<AppState>,
    Path(id):         Path<Uuid>,
    Json(body):       Json<ProductUpdateRequestBody>,
) -> Result<impl IntoResponse, ProductInterError> {
    let product = ProductUpdateHandler::new(app_state.product_repo)
        .execute(ProductUpdateCommand {
            description:         body.description,
            id,
            image_url:           body.image_url,
            low_stock_threshold: body.low_stock_threshold,
            name:                body.name,
            price_cents:         body.price_cents,
            stock:               body.stock,
            symbols:             body.symbols,
            unit_of_measure:     body.unit_of_measure,
        })
        .await?;
    Ok((StatusCode::OK, Json(ProductUpdateResponse::from(product))))
}

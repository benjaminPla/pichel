pub mod dto;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use crate::{
    application::product::create::{CreateProductInput, CreateProductUseCase},
    interfaces::{
        app_state::AppState,
        product::{
            errors::ProductInterError,
            handlers::create::dto::{ProductCreateRequestBody, ProductCreateResponse},
        },
    },
};

pub async fn create(
    State(app_state): State<AppState>,
    Json(body):       Json<ProductCreateRequestBody>,
) -> Result<impl IntoResponse, ProductInterError> {
    let product = CreateProductUseCase::new(app_state.product_repo)
        .execute(CreateProductInput {
            description:     body.description,
            image_url:       body.image_url,
            name:            body.name,
            price_cents:     body.price_cents,
            sale_mode:       body.sale_mode,
            stock:           body.stock,
            symbols:         body.symbols,
            unit_of_measure: body.unit_of_measure,
        })
        .await?;
    Ok((StatusCode::CREATED, Json(ProductCreateResponse::from(product))))
}

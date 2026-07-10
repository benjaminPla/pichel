pub mod dto;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};
use crate::{
    application::product::create::{CreateProductInput, CreateProductUseCase},
    domain::auth::Claims,
    interfaces::{
        app_state::AppState,
        product::{
            errors::ProductInterError,
            handlers::create::dto::{ProductCreateRequestBody, ProductCreateResponse},
        },
    },
};

pub async fn create(
    State(app_state):   State<AppState>,
    Extension(claims):  Extension<Claims>,
    Json(body):         Json<ProductCreateRequestBody>,
) -> Result<impl IntoResponse, ProductInterError> {
    let product = CreateProductUseCase::new(app_state.product_repo)
        .execute(CreateProductInput {
            active:      body.active.unwrap_or(true),
            description: body.description,
            image_url:   body.image_url,
            name:        body.name,
            price_cents: body.price_cents,
            sale_mode:   body.sale_mode,
            symbols:     body.symbols,
            updated_by:  claims.get_sub().value(),
        })
        .await?;
    Ok((StatusCode::CREATED, Json(ProductCreateResponse::from(product))))
}

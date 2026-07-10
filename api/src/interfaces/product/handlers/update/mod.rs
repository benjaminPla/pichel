pub mod dto;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use crate::{
    application::product::update::{UpdateProductInput, UpdateProductUseCase},
    domain::auth::Claims,
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
    State(app_state):  State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id):          Path<Uuid>,
    Json(body):        Json<ProductUpdateRequestBody>,
) -> Result<impl IntoResponse, ProductInterError> {
    let product = UpdateProductUseCase::new(app_state.product_repo)
        .execute(UpdateProductInput {
            description: body.description,
            id,
            active:      body.active,
            image_url:   body.image_url,
            name:        body.name,
            price_cents: body.price_cents,
            sale_mode:   body.sale_mode,
            symbols:     body.symbols,
            updated_by:  claims.get_sub().value(),
        })
        .await?;
    Ok((StatusCode::OK, Json(ProductUpdateResponse::from(product))))
}

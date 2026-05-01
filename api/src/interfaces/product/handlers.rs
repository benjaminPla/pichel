use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    application::product::{
        commands::create::{ProductCreateCommand, ProductCreateHandler},
        queries::get_all::{ProductGetAllHandler, ProductGetAllQuery},
    },
    interfaces::{
        app_state::AppState,
        pagination::{GetAllQueryParams, MAX_PER_PAGE},
    },
};
use super::{
    dto::{
        ProductCreateRequestBody, ProductCreateResponse, ProductGetAllItem, ProductGetAllResponse,
    },
    errors::ProductInterError,
};

// ── Create ───────────────────────────────────────────────────────────────

pub async fn product_create(
    State(app_state): State<AppState>,
    Json(body):       Json<ProductCreateRequestBody>,
) -> Result<impl IntoResponse, ProductInterError> {
    let product = ProductCreateHandler::new(app_state.product_repo)
        .execute(ProductCreateCommand {
            description:         body.description,
            image_url:           body.image_url,
            low_stock_threshold: body.low_stock_threshold,
            name:                body.name,
            price_cents:         body.price_cents,
            stock:               body.stock,
            symbols:             body.symbols,
            unit_of_measure:     body.unit_of_measure,
        })
        .await?;
    let product = ProductCreateResponse::from(product);
    Ok((StatusCode::CREATED, Json(product)))
}


// ── GetAll ───────────────────────────────────────────────────────────────

pub async fn product_get_all(
    State(app_state): State<AppState>,
    Query(query):     Query<GetAllQueryParams>,
) -> Result<impl IntoResponse, ProductInterError> {
    let (products, total) = ProductGetAllHandler::new(app_state.product_repo)
        .execute(ProductGetAllQuery {
            page:     query.page.max(1),
            per_page: query.per_page.clamp(1, MAX_PER_PAGE),
        })
        .await?;
    let products = products.into_iter().map(ProductGetAllItem::from).collect::<Vec<_>>();
    let products = ProductGetAllResponse { products, total };
    Ok((StatusCode::OK, Json(products)))
}

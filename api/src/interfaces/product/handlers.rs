use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;
use super::dto::{
    ProductCreateRequestBody, ProductCreateResponse,
    ProductGetAllItem, ProductGetAllResponse,
    ProductGetByIdResponse,
    ProductUpdateRequestBody, ProductUpdateResponse,
};
use super::errors::ProductInterError;
use crate::application::product::{
    commands::{
        create::{ProductCreateCommand, ProductCreateHandler},
        update::{ProductUpdateCommand, ProductUpdateHandler},
    },
    queries::{
        get_all::{ProductGetAllHandler, ProductGetAllQuery},
        get_by_id::{ProductGetByIdHandler, ProductGetByIdQuery},
    },
};
use crate::interfaces::{
    app_state::AppState,
    pagination::{GetAllQueryParams, MAX_PER_PAGE},
};

// ── Create ───────────────────────────────────────────────────────────────

pub async fn create(
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
    Ok((StatusCode::CREATED, Json(ProductCreateResponse::from(product))))
}

// ── Update ───────────────────────────────────────────────────────────────

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

// ── GetAll ───────────────────────────────────────────────────────────────

pub async fn get_all(
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
    Ok((StatusCode::OK, Json(ProductGetAllResponse { products, total })))
}

// ── GetById ──────────────────────────────────────────────────────────────

pub async fn get_by_id(
    State(app_state): State<AppState>,
    Path(id):         Path<Uuid>,
) -> Result<impl IntoResponse, ProductInterError> {
    let product = ProductGetByIdHandler::new(app_state.product_repo)
        .execute(ProductGetByIdQuery { id })
        .await?;
    Ok((StatusCode::OK, Json(ProductGetByIdResponse::from(product))))
}

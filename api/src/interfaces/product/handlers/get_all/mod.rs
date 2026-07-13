pub mod dto;

use crate::{
    application::product::get_all::{GetAllProductsInput, GetAllProductsUseCase},
    interfaces::{
        app_state::AppState,
        product::{
            errors::ProductInterError,
            handlers::get_all::dto::{GetAllQueryParams, ProductGetAllItem, ProductGetAllResponse},
        },
    },
};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

pub async fn get_all(
    State(app_state): State<AppState>,
    Query(query):     Query<GetAllQueryParams>,
) -> Result<impl IntoResponse, ProductInterError> {
    let (products, total, price_list_updated_at) = GetAllProductsUseCase::new(app_state.product_repo)
        .execute(GetAllProductsInput {
            page:        query.page.max(1),
            per_page:    query.per_page.clamp(1, GetAllQueryParams::MAX_PER_PAGE),
            name:        query.name,
            category_id: query.category_id,
        })
        .await?;
    let products = products.into_iter().map(ProductGetAllItem::from).collect::<Vec<_>>();
    Ok((StatusCode::OK, Json(ProductGetAllResponse {
        products,
        total,
        price_list_updated_at: price_list_updated_at.map(|dt| dt.to_rfc3339()),
    })))
}

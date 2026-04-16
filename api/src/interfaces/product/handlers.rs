use axum::{extract::{Query, State}, Json};
use serde::Deserialize;

use crate::interfaces::{
    errors::AppError,
    middleware::AuthenticatedUser,
    product::dto::{AdminProductResponse, CreateProductRequest, Paginated, PublicProductResponse},
    state::AppState,
};

#[derive(Deserialize)]
pub struct PaginationQuery {
    #[serde(default = "default_page")]
    page:     i64,
    #[serde(default = "default_per_page")]
    per_page: i64,
}

fn default_page()     -> i64 { 1 }
fn default_per_page() -> i64 { 20 }

/// Public — no auth required.
pub async fn list_products(
    State(state): State<AppState>,
    Query(q): Query<PaginationQuery>,
) -> Result<Json<Paginated<PublicProductResponse>>, AppError> {
    let page     = q.page.max(1);
    let per_page = q.per_page.clamp(1, 50);
    let (products, total) = state.list_products_uc.execute(page, per_page).await?;
    let total_pages = (total + per_page - 1) / per_page;
    Ok(Json(Paginated {
        data: products.into_iter().map(PublicProductResponse::from).collect(),
        total,
        page,
        per_page,
        total_pages,
    }))
}

/// Admin — auth required.
pub async fn list_products_admin(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Query(q): Query<PaginationQuery>,
) -> Result<Json<Paginated<AdminProductResponse>>, AppError> {
    let page     = q.page.max(1);
    let per_page = q.per_page.clamp(1, 500);
    let (products, total) = state.list_products_uc.execute(page, per_page).await?;
    let total_pages = (total + per_page - 1) / per_page;
    Ok(Json(Paginated {
        data: products.into_iter().map(AdminProductResponse::from).collect(),
        total,
        page,
        per_page,
        total_pages,
    }))
}

/// Create — auth required.
pub async fn create_product(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Json(body): Json<CreateProductRequest>,
) -> Result<Json<AdminProductResponse>, AppError> {
    let product = state.create_product_uc.execute(
        body.name,
        body.description,
        body.price_cents,
        body.cost_price,
        body.unit_amount,
        body.unit_type,
        body.stock,
        body.low_stock_threshold,
        body.image_url,
    ).await?;
    Ok(Json(AdminProductResponse::from(product)))
}

use axum::{extract::State, Json};

use crate::interfaces::{
    errors::AppError,
    middleware::AuthenticatedUser,
    product::dto::{AdminProductResponse, CreateProductRequest, PublicProductResponse},
    state::AppState,
};

/// Public — no auth required. Returns stock_status but never cost_price.
pub async fn list_products(
    State(state): State<AppState>,
) -> Result<Json<Vec<PublicProductResponse>>, AppError> {
    let products = state.list_products_uc.execute().await?;
    Ok(Json(products.into_iter().map(PublicProductResponse::from).collect()))
}

/// Admin — auth required. Returns all fields including cost_price and raw stock.
pub async fn list_products_admin(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
) -> Result<Json<Vec<AdminProductResponse>>, AppError> {
    let products = state.list_products_uc.execute().await?;
    Ok(Json(products.into_iter().map(AdminProductResponse::from).collect()))
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

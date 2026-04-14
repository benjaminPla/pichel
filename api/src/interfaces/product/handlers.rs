use axum::{extract::State, Json};

use crate::interfaces::{
    errors::AppError,
    middleware::AuthenticatedUser,
    product::dto::{CreateProductRequest, ProductResponse},
    state::AppState,
};

pub async fn list_products(
    State(state): State<AppState>,
) -> Result<Json<Vec<ProductResponse>>, AppError> {
    let products = state.list_products_uc.execute().await?;
    Ok(Json(products.into_iter().map(ProductResponse::from).collect()))
}

pub async fn create_product(
    State(state): State<AppState>,
    _user: AuthenticatedUser, // ← declaring this param IS the auth check — no JWT code here
    Json(body): Json<CreateProductRequest>,
) -> Result<Json<ProductResponse>, AppError> {
    let product = state.create_product_uc.execute(body.name, body.price).await?;
    Ok(Json(ProductResponse::from(product)))
}

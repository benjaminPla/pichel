use axum::{routing::{get, post}, Router};

use crate::interfaces::{
    auth::handlers::{login, register},
    product::handlers::{create_product, list_products},
    state::AppState,
};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .nest("/auth",     auth_routes())
        .nest("/products", product_routes())
        .with_state(state)
}

fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login",    post(login))
}

fn product_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list_products).post(create_product))
}

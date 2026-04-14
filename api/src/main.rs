mod application;
mod domain;
mod infrastructure;
mod interfaces;

use std::sync::Arc;

use infrastructure::{
    auth::{JwtTokenService, PostgresUserRepository},
    db::create_pool,
    product::PostgresProductRepository,
};
use application::{
    auth::{LoginUseCase, RegisterUseCase},
    product::{CreateProductUseCase, ListProductsUseCase},
};
use interfaces::{router::create_router, state::AppState};

#[tokio::main]
async fn main() {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let jwt_secret   = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let pool = create_pool(&database_url).await.expect("failed to connect to db");

    // ── Infrastructure (adapters) ──────────────────────────────────────────
    let user_repo    = Arc::new(PostgresUserRepository::new(pool.clone()));
    let product_repo = Arc::new(PostgresProductRepository::new(pool.clone()));
    let token_service: Arc<dyn domain::auth::service::TokenService> =
        Arc::new(JwtTokenService::new(jwt_secret));

    // ── Application (use cases) ────────────────────────────────────────────
    let register_uc       = Arc::new(RegisterUseCase::new(user_repo.clone(), token_service.clone()));
    let login_uc          = Arc::new(LoginUseCase::new(user_repo.clone(), token_service.clone()));
    let create_product_uc = Arc::new(CreateProductUseCase::new(product_repo.clone()));
    let list_products_uc  = Arc::new(ListProductsUseCase::new(product_repo.clone()));

    // ── Interfaces (HTTP) ──────────────────────────────────────────────────
    let state = AppState { token_service, register_uc, login_uc, create_product_uc, list_products_uc };
    let app   = create_router(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}

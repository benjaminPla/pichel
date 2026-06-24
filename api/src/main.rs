mod application;
mod config;
mod domain;
mod infrastructure;
mod interfaces;

use axum::Router;
use config::Config;
use infrastructure::{
    argon2_hasher_service::Argon2HasherService, jwt_token_service::JwtTokenService,
    pg_order_repo::PgOrderRepo, pg_product_repo::PgProductRepo, pg_user_repo::PgUserRepo,
};
use interfaces::{
    app_state::AppState,
    auth::router::auth_router,
    order::router::orders_router,
    product::router::products_router,
    user::router::users_router,
};
use std::{net::SocketAddr, sync::Arc};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    let config = envy::from_env::<Config>().expect("Failed to load env vars");
    let pool   = sqlx::PgPool::connect(&config.database_url).await.expect("Failed to connect to Postgres");
    sqlx::migrate!("../db/migrations").run(&pool).await.expect("Failed to run migrations");
    tokio::fs::create_dir_all(&config.uploads_path).await.expect("Failed to create uploads directory");
    let app_state = AppState {
        cookie_secure:  config.cookie_secure,
        hasher_service: Arc::new(Argon2HasherService::new()),
        order_repo:     Arc::new(PgOrderRepo::new(pool.clone())),
        token_service:  Arc::new(JwtTokenService::new(config.jwt_secret)),
        product_repo:   Arc::new(PgProductRepo::new(pool.clone())),
        uploads_path:   config.uploads_path,
        user_repo:      Arc::new(PgUserRepo::new(pool.clone())),
    };
    let app = Router::new()
        .nest("/auth",     auth_router())
        .nest("/orders",   orders_router(app_state.clone()))
        .nest("/products", products_router(app_state.clone()))
        .nest("/users",    users_router(app_state.clone()))
        .with_state(app_state);
    let address  = SocketAddr::from(([0, 0, 0, 0], config.port));
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    tracing::info!("listening on {address}");
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

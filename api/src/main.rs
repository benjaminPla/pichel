mod application;
mod config;
mod domain;
mod infrastructure;
mod interfaces;

use axum::Router;
use config::Config;
use infrastructure::{
    argon2_hasher_service::Argon2HasherService, jwt_token_service::JwtTokenService,
    pg_product_repo::PgProductRepo, pg_user_repo::PgUserRepo,
};
use interfaces::{
    app_state::AppState, product::router::products_router, user::router::users_router,
};
use std::{net::SocketAddr, sync::Arc};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    let config = envy::from_env::<Config>().expect("Failed to load env vars");
    let pool   = sqlx::PgPool::connect(&config.database_url).await.expect("Failed to connect to Postgres");
    let app_state = AppState {
        hasher_service: Arc::new(Argon2HasherService::new()),
        token_service:  Arc::new(JwtTokenService::new(config.jwt_secret)),
        product_repo:   Arc::new(PgProductRepo::new(pool.clone())),
        user_repo:      Arc::new(PgUserRepo::new(pool.clone())),
    };
    let app = Router::new()
        .nest("/products", products_router())
        .nest("/users",    users_router())
        .with_state(app_state);
    let address  = SocketAddr::from(([0, 0, 0, 0], config.port));
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    tracing::info!("listening on {address}");
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

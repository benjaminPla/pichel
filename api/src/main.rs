mod application;
mod config;
mod domain;
mod infrastructure;
mod interfaces;

use std::{net::SocketAddr, sync::Arc};

use axum::Router;

use config::Config;
use infrastructure::{
    product::postgres::repository::PgProductRepo,
    user::{argon2::Argon2Hasher, postgres::repository::PgUserRepo},
};
use interfaces::{
    app_state::AppState, product::router::products_router, user::router::users_router,
};

#[tokio::main]
async fn main() {
    let config = envy::from_env::<Config>().expect("Failed to load env vars");
    let pool = sqlx::PgPool::connect(&config.database_url).await.expect("Failed to connect to Postgres");
    let app_state = AppState {
        product_repo: Arc::new(PgProductRepo::new(pool.clone())),
        user_hasher:  Arc::new(Argon2Hasher::new()),
        user_repo:    Arc::new(PgUserRepo::new(pool.clone())),
    };
    let app = Router::new()
        .nest("/products", products_router())
        .nest("/users", users_router())
        .with_state(app_state);
    let address = SocketAddr::from(([0, 0, 0, 0], config.port));
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    println!("listening on {}", address.to_string());
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

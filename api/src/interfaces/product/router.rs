use axum::{routing::{get, post}, Router};

use crate::interfaces::{app_state::AppState, product::handlers};

pub fn products_router() -> Router<AppState> {
    Router::new()
        .route("/", post(handlers::product_create))
        .route("/", get(handlers::product_get_all))
}

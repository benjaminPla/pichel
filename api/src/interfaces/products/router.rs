use axum::{routing::{get, post}, Router};

use crate::interfaces::{products::handlers,app_state::AppState};

pub fn products_router() -> Router<AppState> {
    Router::new().route("/", post(handlers::product_create)).route("/", get(handlers::product_get_all))
}

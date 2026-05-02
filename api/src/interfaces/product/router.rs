use axum::{routing::{get, patch, post}, Router};
use crate::interfaces::{app_state::AppState, product::handlers};

pub fn products_router() -> Router<AppState> {
    Router::new()
        .route("/",    get(handlers::get_all))
        .route("/:id", get(handlers::get_by_id))
        .route("/:id", patch(handlers::update))
        .route("/",    post(handlers::create))
}

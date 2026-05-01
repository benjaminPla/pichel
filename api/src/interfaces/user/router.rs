use axum::{routing::{get, post}, Router};

use crate::interfaces::{app_state::AppState, user::handlers};

pub fn users_router() -> Router<AppState> {
    Router::new()
        .route("/",     post(handlers::create))
        .route("/",     get(handlers::get_all))
        .route("/:id",  get(handlers::get_by_id))
}

use axum::{routing::{get, post}, Router};

use crate::interfaces::{app_state::AppState, user::handlers};

pub fn users_router() -> Router<AppState> {
    Router::new()
        .route("/", post(handlers::user_create))
        .route("/", get(handlers::user_get_all))
}

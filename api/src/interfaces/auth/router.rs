use axum::{routing::post, Router};
use crate::interfaces::{app_state::AppState, auth::handlers};

pub fn auth_router() -> Router<AppState> {
    Router::new()
        .route("/authenticate", post(handlers::authenticate))
}

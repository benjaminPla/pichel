use crate::interfaces::{app_state::AppState, middleware::auth_middleware, user::handlers};
use axum::{
    middleware,
    routing::{get, patch, post},
    Router,
};

pub fn users_router(state: AppState) -> Router<AppState> {
    let admin = Router::new()
        .route("/",             get(handlers::get_all))
        .route("/:id",          get(handlers::get_by_id))
        .route("/:id/password", patch(handlers::update_password))
        .route("/:id",          patch(handlers::update))
        .route("/",             post(handlers::create))
        .route_layer(middleware::from_fn_with_state(state, auth_middleware));

    Router::new()
        .merge(admin)
}

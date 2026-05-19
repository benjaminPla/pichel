use crate::interfaces::{app_state::AppState, middleware::auth_middleware, order::handlers};
use axum::{
    middleware,
    routing::{get, post},
    Router,
};

pub fn orders_router(app_state: AppState) -> Router<AppState> {
    let admin = Router::new()
        .route("/", get(handlers::get_all))
        .route_layer(middleware::from_fn_with_state(app_state, auth_middleware));

    let public = Router::new()
        .route("/",    post(handlers::create))
        .route("/:id", get(handlers::get_by_id));

    Router::new()
        .merge(admin)
        .merge(public)
}

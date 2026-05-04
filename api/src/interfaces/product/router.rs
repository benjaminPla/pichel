use axum::{middleware, routing::{get, patch, post}, Router};
use crate::interfaces::{app_state::AppState, auth::auth_middleware, product::handlers};

pub fn products_router(app_state: AppState) -> Router<AppState> {
    let admin = Router::new()
        .route("/",    post(handlers::create))
        .route("/:id", patch(handlers::update))
        .route_layer(middleware::from_fn_with_state(app_state, auth_middleware));

    let public = Router::new()
        .route("/",    get(handlers::get_all))
        .route("/:id", get(handlers::get_by_id));

    Router::new()
        .merge(admin)
        .merge(public)
}

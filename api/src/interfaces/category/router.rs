use crate::interfaces::{app_state::AppState, category::handlers, middleware::auth_middleware};
use axum::{
    middleware,
    routing::{delete, get, patch, post},
    Router,
};

pub fn categories_router(app_state: AppState) -> Router<AppState> {
    let admin = Router::new()
        .route("/",    get(handlers::get_all))
        .route("/",    post(handlers::create))
        .route("/:id", patch(handlers::update))
        .route("/:id", delete(handlers::delete))
        .route_layer(middleware::from_fn_with_state(app_state, auth_middleware));

    Router::new().merge(admin)
}

use crate::interfaces::{app_state::AppState, middleware::auth_middleware, product::handlers};
use axum::{
    middleware,
    routing::{delete, get, patch, post},
    Router,
};

pub fn products_router(app_state: AppState) -> Router<AppState> {
    let admin = Router::new()
        .route("/:id",  delete(handlers::delete))
        .route("/:id",  patch(handlers::update))
        .route("/:id",  get(handlers::get_by_id))
        .route("/",     post(handlers::create))
        .route("/admin", get(handlers::get_all_admin))
        .route("/image", post(handlers::upload_image))
        .route_layer(middleware::from_fn_with_state(app_state, auth_middleware));

    let public = Router::new()
        .route("/", get(handlers::get_all));

    Router::new()
        .merge(admin)
        .merge(public)
}

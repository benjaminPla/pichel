use crate::domain::{
    auth::ports::TokenService,
    category::ports::repository::CategoryRepo,
    hasher::HasherService,
    order::ports::repository::OrderRepo,
    product::ports::repository::ProductRepo,
    user::ports::UserRepo,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub category_repo: Arc<dyn CategoryRepo>,
    pub cookie_secure:  bool,
    pub hasher_service: Arc<dyn HasherService>,
    pub order_repo:     Arc<dyn OrderRepo>,
    pub product_repo:   Arc<dyn ProductRepo>,
    pub token_service:  Arc<dyn TokenService>,
    pub uploads_path:   String,
    pub user_repo:      Arc<dyn UserRepo>,
}

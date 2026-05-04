use crate::domain::{
    auth::ports::TokenService, hasher::HasherService, product::ports::repository::ProductRepo,
    user::ports::UserRepo,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub hasher_service: Arc<dyn HasherService>,
    pub product_repo:   Arc<dyn ProductRepo>,
    pub token_service:  Arc<dyn TokenService>,
    pub user_repo:      Arc<dyn UserRepo>,
}

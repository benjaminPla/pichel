use std::sync::Arc;
use crate::domain::{
    product::ports::repository::ProductRepo,
    user::ports::{hasher::Hasher, repository::UserRepo},
};

#[derive(Clone)]
pub struct AppState {
    pub product_repo: Arc<dyn ProductRepo>,
    pub user_hasher:  Arc<dyn Hasher>,
    pub user_repo:    Arc<dyn UserRepo>,
}

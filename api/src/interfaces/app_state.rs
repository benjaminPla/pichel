use std::sync::Arc;

use crate::domain::{
    product::repository::ProductRepo,
    user::{ports::user_hasher::UserHasher, repository::UserRepo},
};

#[derive(Clone)]
pub struct AppState {
    pub product_repo: Arc<dyn ProductRepo>,
    pub user_hasher:  Arc<dyn UserHasher>,
    pub user_repo:    Arc<dyn UserRepo>,
}

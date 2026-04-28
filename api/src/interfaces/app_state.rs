use std::sync::Arc;

use crate::domain::{product::repository::ProductRepo, user::repository::UserRepo};

#[derive(Clone)]
pub struct AppState {
    pub product_repo: Arc<dyn ProductRepo>,
    pub user_repo:    Arc<dyn UserRepo>
}

use std::sync::Arc;

use crate::domain::product::repository::ProductRepo;

#[derive(Clone)]
pub struct AppState {
    pub product_repo: Arc<dyn ProductRepo>
}

use std::sync::Arc;
use crate::domain::product::{Product, ProductDomainError, ProductRepository};

pub struct ListProductsUseCase {
    product_repo: Arc<dyn ProductRepository>,
}

impl ListProductsUseCase {
    pub fn new(product_repo: Arc<dyn ProductRepository>) -> Self {
        Self { product_repo }
    }

    pub async fn execute(&self, page: i64, per_page: i64) -> Result<(Vec<Product>, i64), ProductDomainError> {
        self.product_repo.find_all(page, per_page).await
    }
}

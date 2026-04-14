use std::sync::Arc;
use crate::domain::product::{Product, ProductDomainError, ProductRepository};

pub struct ListProductsUseCase {
    product_repo: Arc<dyn ProductRepository>,
}

impl ListProductsUseCase {
    pub fn new(product_repo: Arc<dyn ProductRepository>) -> Self {
        Self { product_repo }
    }

    pub async fn execute(&self) -> Result<Vec<Product>, ProductDomainError> {
        self.product_repo.find_all().await
    }
}

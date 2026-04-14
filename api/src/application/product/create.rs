use std::sync::Arc;
use crate::domain::product::{Price, Product, ProductDomainError, ProductName, ProductRepository};

pub struct CreateProductUseCase {
    product_repo: Arc<dyn ProductRepository>,
}

impl CreateProductUseCase {
    pub fn new(product_repo: Arc<dyn ProductRepository>) -> Self {
        Self { product_repo }
    }

    pub async fn execute(&self, name: String, price: f64) -> Result<Product, ProductDomainError> {
        let product = Product::create(ProductName::new(name)?, Price::new(price)?);
        self.product_repo.save(&product).await?;
        Ok(product)
    }
}

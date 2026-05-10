use crate::{
    application::product::errors::ProductAppError,
    domain::product::{ports::repository::ProductRepo, Product},
};
use std::sync::Arc;

pub struct GetAllProductsInput {
    pub page:     i64,
    pub per_page: i64,
}

pub struct GetAllProductsUseCase {
    product_repo: Arc<dyn ProductRepo>,
}

impl GetAllProductsUseCase {
    pub fn new(product_repo: Arc<dyn ProductRepo>) -> Self {
        Self { product_repo }
    }

    pub async fn execute(&self, input: GetAllProductsInput) -> Result<(Vec<Product>, i64), ProductAppError> {
        let (products, total) = self.product_repo.get_all(input.page, input.per_page).await?;
        Ok((products, total))
    }
}

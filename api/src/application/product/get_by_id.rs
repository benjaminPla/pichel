use crate::{
    application::product::errors::ProductAppError,
    domain::product::{ports::repository::ProductRepo, value_objects::id::ProductId, Product},
};
use std::sync::Arc;
use uuid::Uuid;

pub struct GetProductByIdInput {
    pub id: Uuid,
}

pub struct GetProductByIdUseCase {
    product_repo: Arc<dyn ProductRepo>,
}

impl GetProductByIdUseCase {
    pub fn new(product_repo: Arc<dyn ProductRepo>) -> Self {
        Self { product_repo }
    }

    pub async fn execute(&self, input: GetProductByIdInput) -> Result<Product, ProductAppError> {
        let product_id = ProductId::reconstitute(input.id);
        let product    = self.product_repo.get_by_id(&product_id).await?;
        Ok(product)
    }
}

use crate::{
    application::product::errors::ProductAppError,
    domain::product::{ports::repository::ProductRepo, value_objects::id::ProductId},
};
use std::sync::Arc;
use uuid::Uuid;

pub struct DeleteProductInput {
    pub id:         Uuid,
    pub updated_by: Uuid,
}

pub struct DeleteProductUseCase {
    product_repo: Arc<dyn ProductRepo>,
}

impl DeleteProductUseCase {
    pub fn new(product_repo: Arc<dyn ProductRepo>) -> Self {
        Self { product_repo }
    }

    pub async fn execute(&self, input: DeleteProductInput) -> Result<(), ProductAppError> {
        let product_id = ProductId::reconstitute(input.id);
        self.product_repo.delete(&product_id, input.updated_by).await?;
        Ok(())
    }
}

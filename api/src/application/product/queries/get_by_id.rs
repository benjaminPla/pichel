use std::sync::Arc;
use uuid::Uuid;
use crate::{
    application::product::errors::ProductAppError,
    domain::product::{
        aggregate_root::Product,
        ports::repository::ProductRepo,
        value_objects::id::ProductId,
    },
};

// ── Query ────────────────────────────────────────────────────────────────

pub struct ProductGetByIdQuery {
    pub id: Uuid,
}

// ── Handler ──────────────────────────────────────────────────────────────

pub struct ProductGetByIdHandler {
    product_repo: Arc<dyn ProductRepo>,
}

impl ProductGetByIdHandler {
    pub fn new(product_repo: Arc<dyn ProductRepo>) -> Self {
        Self { product_repo }
    }

    pub async fn execute(&self, query: ProductGetByIdQuery) -> Result<Product, ProductAppError> {
        let product_id = ProductId::reconstitute(query.id);
        let product    = self.product_repo.get_by_id(&product_id).await?;
        Ok(product)
    }
}

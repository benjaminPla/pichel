use async_trait::async_trait;

use super::{aggregate_root::Product, value_objects::id::ProductId};

#[async_trait]
pub trait ProductRepo: Send + Sync {
    async fn get_all(&self, page: i64, per_page: i64) -> Result<(Vec<Product>, i64), ProductRepoError>;
    async fn get_by_id(&self, product_id: &ProductId) -> Result<Product, ProductRepoError>;
    async fn save(&self, product: &Product)           -> Result<Product, ProductRepoError>;
}

// ── Errors ───────────────────────────────────────────────────────────────

#[derive(Debug, thiserror::Error)]
pub enum ProductRepoError {
    #[error("internal server error")]
    Database,
    #[error("{0}")]
    Mapping(String),
}

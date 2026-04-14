use async_trait::async_trait;
use super::product::{Product, ProductDomainError, ProductId};

#[async_trait]
pub trait ProductRepository: Send + Sync {
    async fn save(&self, product: &Product) -> Result<(), ProductDomainError>;
    async fn find_by_id(&self, id: &ProductId) -> Result<Option<Product>, ProductDomainError>;
    async fn find_all(&self) -> Result<Vec<Product>, ProductDomainError>;
}

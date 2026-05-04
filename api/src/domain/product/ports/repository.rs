use crate::domain::product::{
    value_objects::{
        description::DescriptionError, id::ProductId, name::NameError, symbol::SymbolError,
        unit_of_measure::UnitOfMeasureError,
    },
    Product,
};
use async_trait::async_trait;

#[async_trait]
pub trait ProductRepo: Send + Sync {
    async fn create(&self, product: &Product)          -> Result<Product, ProductRepoError>;
    async fn get_all(&self, page: i64, per_page: i64)  -> Result<(Vec<Product>, i64), ProductRepoError>;
    async fn get_by_id(&self, product_id: &ProductId)  -> Result<Product, ProductRepoError>;
    async fn update(&self, product: &Product)           -> Result<Product, ProductRepoError>;
}

// ── Errors ───────────────────────────────────────────────────────────────

#[derive(Debug, thiserror::Error)]
pub enum ProductRepoError {
    #[error("internal server error")]
    Database(String),
    #[error("not found")]
    NotFound,
    #[error("{0}")]
    Mapping(String),
}

impl From<DescriptionError> for ProductRepoError {
    fn from(e: DescriptionError) -> Self { Self::Mapping(e.to_string()) }
}

impl From<NameError> for ProductRepoError {
    fn from(e: NameError) -> Self { Self::Mapping(e.to_string()) }
}

impl From<SymbolError> for ProductRepoError {
    fn from(e: SymbolError) -> Self { Self::Mapping(e.to_string()) }
}

impl From<UnitOfMeasureError> for ProductRepoError {
    fn from(e: UnitOfMeasureError) -> Self { Self::Mapping(e.to_string()) }
}

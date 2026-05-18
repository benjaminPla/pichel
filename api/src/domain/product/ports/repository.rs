use crate::domain::product::{
    value_objects::{
        description::DescriptionError,
        id::ProductId,
        name::NameError,
        price_cents::PriceCentsError,
        sale_mode::SaleModeError,
        symbol::SymbolError,
        unit_of_measure::UnitOfMeasureError,
    },
    Product,
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};

#[async_trait]
pub trait ProductRepo: Send + Sync {
    async fn create(&self, product: &Product)         -> Result<Product, ProductRepoError>;
    async fn delete(&self, product: &ProductId)       -> Result<Product, ProductRepoError>;
    async fn get_all(&self, page: i64, per_page: i64) -> Result<(Vec<Product>, i64, Option<DateTime<Utc>>), ProductRepoError>;
    async fn get_by_id(&self, product_id: &ProductId) -> Result<Product, ProductRepoError>;
    async fn update(&self, product: &Product)         -> Result<Product, ProductRepoError>;
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

impl From<PriceCentsError> for ProductRepoError {
    fn from(e: PriceCentsError) -> Self { Self::Mapping(e.to_string()) }
}

impl From<SaleModeError> for ProductRepoError {
    fn from(e: SaleModeError) -> Self { Self::Mapping(e.to_string()) }
}

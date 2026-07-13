use crate::domain::category::{value_objects::{id::CategoryId, name::NameError}, Category};
use async_trait::async_trait;

#[async_trait]
pub trait CategoryRepo: Send + Sync {
    async fn create(&self, category: &Category)      -> Result<Category, CategoryRepoError>;
    async fn delete(&self, category_id: &CategoryId) -> Result<(), CategoryRepoError>;
    async fn get_all(&self)                          -> Result<Vec<Category>, CategoryRepoError>;
    async fn update(&self, category: &Category)      -> Result<Category, CategoryRepoError>;
}

// ── Errors ───────────────────────────────────────────────────────────────

#[derive(Debug, thiserror::Error)]
pub enum CategoryRepoError {
    #[error("internal server error")]
    Database(String),
    #[error("not found")]
    NotFound,
    #[error("{0}")]
    Mapping(String),
    #[error("{0}")]
    Validation(String),
}

impl From<NameError> for CategoryRepoError {
    fn from(e: NameError) -> Self { Self::Mapping(e.to_string()) }
}

use crate::domain::product::{
    ports::repository::ProductRepoError,
    value_objects::{
        description::DescriptionError, name::NameError, symbol::SymbolError,
        unit_of_measure::UnitOfMeasureError,
    },
};

#[derive(Debug, thiserror::Error)]
pub enum ProductAppError {
    #[error("internal server error")]
    Internal(String),
    #[error("not found")]
    NotFound,
    #[error("{0}")]
    Validation(String),
}

impl From<DescriptionError> for ProductAppError {
    fn from(e: DescriptionError) -> Self { Self::Validation(e.to_string()) }
}

impl From<NameError> for ProductAppError {
    fn from(e: NameError) -> Self { Self::Validation(e.to_string()) }
}

impl From<SymbolError> for ProductAppError {
    fn from(e: SymbolError) -> Self { Self::Validation(e.to_string()) }
}

impl From<UnitOfMeasureError> for ProductAppError {
    fn from(e: UnitOfMeasureError) -> Self { Self::Validation(e.to_string()) }
}

impl From<ProductRepoError> for ProductAppError {
    fn from(e: ProductRepoError) -> Self {
        match e {
            ProductRepoError::Database(msg) => Self::Internal(msg),
            ProductRepoError::Mapping(msg)  => Self::Internal(msg),
            ProductRepoError::NotFound      => Self::NotFound,
        }
    }
}

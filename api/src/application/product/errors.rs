use crate::domain::product::{
    ports::repository::ProductRepoError,
    value_objects::{
        name::NameError,
        price_cents::PriceCentsError,
        sale_mode::SaleModeError,
        symbol::SymbolError,
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

impl From<NameError> for ProductAppError {
    fn from(e: NameError) -> Self { Self::Validation(e.to_string()) }
}

impl From<PriceCentsError> for ProductAppError {
    fn from(e: PriceCentsError) -> Self { Self::Validation(e.to_string()) }
}

impl From<SaleModeError> for ProductAppError {
    fn from(e: SaleModeError) -> Self { Self::Validation(e.to_string()) }
}

impl From<SymbolError> for ProductAppError {
    fn from(e: SymbolError) -> Self { Self::Validation(e.to_string()) }
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

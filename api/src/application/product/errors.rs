use crate::domain::product::{
    ports::repository::ProductRepoError,
    value_objects::{
        description::DescriptionError, name::NameError, symbol::SymbolError,
        unit_of_measure::UnitOfMeasureError,
    },
};

#[derive(Debug, thiserror::Error)]
pub enum ProductAppError {
    #[error(transparent)]
    Description(#[from] DescriptionError),
    #[error(transparent)]
    Name(#[from] NameError),
    #[error(transparent)]
    Repo(#[from] ProductRepoError),
    #[error(transparent)]
    Symbol(#[from] SymbolError),
    #[error(transparent)]
    UnitOfMeasure(#[from] UnitOfMeasureError),
}

use crate::domain::category::{ports::repository::CategoryRepoError, value_objects::name::NameError};

#[derive(Debug, thiserror::Error)]
pub enum CategoryAppError {
    #[error("internal server error")]
    Internal(String),
    #[error("not found")]
    NotFound,
    #[error("{0}")]
    Validation(String),
}

impl From<NameError> for CategoryAppError {
    fn from(e: NameError) -> Self { Self::Validation(e.to_string()) }
}

impl From<CategoryRepoError> for CategoryAppError {
    fn from(e: CategoryRepoError) -> Self {
        match e {
            CategoryRepoError::Database(msg)   => Self::Internal(msg),
            CategoryRepoError::Mapping(msg)    => Self::Internal(msg),
            CategoryRepoError::NotFound        => Self::NotFound,
            CategoryRepoError::Validation(msg) => Self::Validation(msg),
        }
    }
}

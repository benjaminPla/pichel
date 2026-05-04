use crate::domain::product::ports::repository::ProductRepoError;

impl From<sqlx::Error> for ProductRepoError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => ProductRepoError::NotFound,
            _                        => ProductRepoError::Database(e.to_string()),
        }
    }
}

impl From<std::num::TryFromIntError> for ProductRepoError {
    fn from(err: std::num::TryFromIntError) -> Self { ProductRepoError::Mapping(err.to_string()) }
}

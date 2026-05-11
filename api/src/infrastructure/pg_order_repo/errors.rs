use crate::domain::order::ports::repository::OrderRepoError;

impl From<sqlx::Error> for OrderRepoError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => OrderRepoError::NotFound,
            _                        => OrderRepoError::Database(e.to_string()),
        }
    }
}

impl From<std::num::TryFromIntError> for OrderRepoError {
    fn from(e: std::num::TryFromIntError) -> Self { OrderRepoError::Mapping(e.to_string()) }
}

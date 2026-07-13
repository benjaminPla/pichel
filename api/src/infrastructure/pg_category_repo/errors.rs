use crate::domain::category::ports::repository::CategoryRepoError;

impl From<sqlx::Error> for CategoryRepoError {
    fn from(e: sqlx::Error) -> Self {
        match &e {
            sqlx::Error::RowNotFound => CategoryRepoError::NotFound,
            sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23505") => {
                CategoryRepoError::Validation("a category with this name already exists".to_string())
            }
            _ => CategoryRepoError::Database(e.to_string()),
        }
    }
}

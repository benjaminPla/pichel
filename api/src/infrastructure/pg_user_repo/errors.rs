use crate::domain::user::ports::UserRepoError;

impl From<sqlx::Error> for UserRepoError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => UserRepoError::NotFound,
            _                        => UserRepoError::Database(e.to_string()),
        }
    }
}

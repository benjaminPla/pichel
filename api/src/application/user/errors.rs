use crate::domain::user::{
    ports::hasher::HasherError,
    ports::repository::UserRepoError,
    value_objects::{email::EmailError, password_raw::PasswordRawError},
};

#[derive(Debug, thiserror::Error)]
pub enum UserAppError {
    #[error("internal server error")]
    Internal,
    #[error("not found")]
    NotFound,
    #[error("{0}")]
    Validation(String),
}

impl From<EmailError> for UserAppError {
    fn from(e: EmailError) -> Self { Self::Validation(e.to_string()) }
}

impl From<HasherError> for UserAppError {
    fn from(_: HasherError) -> Self { Self::Internal }
}

impl From<PasswordRawError> for UserAppError {
    fn from(e: PasswordRawError) -> Self { Self::Validation(e.to_string()) }
}

impl From<UserRepoError> for UserAppError {
    fn from(e: UserRepoError) -> Self {
        match e {
            UserRepoError::Database   => Self::Internal,
            UserRepoError::Mapping(_) => Self::Internal,
            UserRepoError::NotFound   => Self::NotFound,
        }
    }
}

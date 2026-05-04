use crate::domain::{
    hasher::HasherServiceError,
    user::{
        ports::UserRepoError,
        value_objects::{email::EmailError, password_raw::PasswordRawError},
    },
};

#[derive(Debug, thiserror::Error)]
pub enum UserAppError {
    #[error("internal server error")]
    Internal(String),
    #[error("not found")]
    NotFound,
    #[error("{0}")]
    Validation(String),
}

impl From<EmailError> for UserAppError {
    fn from(e: EmailError) -> Self { Self::Validation(e.to_string()) }
}

impl From<HasherServiceError> for UserAppError {
    fn from(e: HasherServiceError) -> Self { Self::Internal(e.to_string()) }
}

impl From<PasswordRawError> for UserAppError {
    fn from(e: PasswordRawError) -> Self { Self::Validation(e.to_string()) }
}

impl From<UserRepoError> for UserAppError {
    fn from(e: UserRepoError) -> Self {
        match e {
            UserRepoError::Database(msg) => Self::Internal(msg),
            UserRepoError::Mapping(msg)  => Self::Internal(msg),
            UserRepoError::NotFound      => Self::NotFound,
        }
    }
}

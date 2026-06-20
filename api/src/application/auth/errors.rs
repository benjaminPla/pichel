use crate::domain::{
    auth::ports::TokenServiceError,
    hasher::HasherServiceError,
    shared::value_objects::email::EmailError,
    user::{
        ports::UserRepoError,
        value_objects::password_raw::PasswordRawError,
    },
};

#[derive(Debug, thiserror::Error)]
pub enum AuthAppError {
    #[error("internal server error")]
    Internal(String),
    #[error("unauthorized")]
    Unauthorized,
}

impl From<EmailError> for AuthAppError {
    fn from(_: EmailError) -> Self { Self::Unauthorized }
}

impl From<PasswordRawError> for AuthAppError {
    fn from(_: PasswordRawError) -> Self { Self::Unauthorized }
}

impl From<HasherServiceError> for AuthAppError {
    fn from(e: HasherServiceError) -> Self { Self::Internal(e.to_string()) }
}

impl From<TokenServiceError> for AuthAppError {
    fn from(e: TokenServiceError) -> Self { Self::Internal(e.to_string()) }
}

impl From<UserRepoError> for AuthAppError {
    fn from(e: UserRepoError) -> Self {
        match e {
            UserRepoError::NotFound      => Self::Unauthorized,
            UserRepoError::Database(msg) => Self::Internal(msg),
            UserRepoError::Mapping(msg)  => Self::Internal(msg),
        }
    }
}

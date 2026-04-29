use crate::domain::user::{
    ports::user_hasher::UserHasherError,
    repository::UserRepoError,
    value_objects::{email::EmailError, password_raw::PasswordRawError},
};

#[derive(Debug, thiserror::Error)]
pub enum UserAppError {
    #[error(transparent)]
    Email(#[from] EmailError),
    #[error(transparent)]
    PasswordRaw(#[from] PasswordRawError),
    #[error(transparent)]
    Repo(#[from] UserRepoError),
    #[error(transparent)]
    UserHasherError(#[from] UserHasherError),
}

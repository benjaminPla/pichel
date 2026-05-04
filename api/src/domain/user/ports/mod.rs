use crate::domain::user::{
    value_objects::{
        email::{Email, EmailError},
        id::UserId,
        password_hash::PasswordHash,
    },
    User,
};
use async_trait::async_trait;

#[async_trait]
pub trait UserRepo: Send + Sync {
    async fn create(&self, user: &User)                                             -> Result<User, UserRepoError>;
    async fn get_all(&self, page: i64, per_page: i64)                               -> Result<(Vec<User>, i64), UserRepoError>;
    async fn get_by_email(&self, email: &Email)                                     -> Result<User, UserRepoError>;
    async fn get_by_id(&self, user_id: &UserId)                                     -> Result<User, UserRepoError>;
    async fn update(&self, user: &User)                                             -> Result<User, UserRepoError>;
    async fn update_password(&self, user_id: &UserId, password_hash: &PasswordHash) -> Result<User, UserRepoError>;
}

#[derive(Debug, thiserror::Error)]
pub enum UserRepoError {
    #[error("internal server error")]
    Database(String),
    #[error("not found")]
    NotFound,
    #[error("{0}")]
    Mapping(String),
}

impl From<EmailError> for UserRepoError {
    fn from(e: EmailError) -> Self { Self::Mapping(e.to_string()) }
}

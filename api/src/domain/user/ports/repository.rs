use async_trait::async_trait;

use crate::domain::user::{aggregate_root::User, value_objects::id::UserId};

#[async_trait]
pub trait UserRepo: Send + Sync {
    async fn get_all(&self, page: i64, per_page: i64) -> Result<(Vec<User>, i64), UserRepoError>;
    async fn get_by_id(&self, user_id: &UserId)       -> Result<User, UserRepoError>;
    async fn save(&self, user: &User)                 -> Result<User, UserRepoError>;
}

#[derive(Debug, thiserror::Error)]
pub enum UserRepoError {
    #[error("internal server error")]
    Database,
    #[error("{0}")]
    Mapping(String),
}

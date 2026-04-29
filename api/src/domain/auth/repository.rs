use async_trait::async_trait;

use crate::domain::user::value_objects::id::UserId;

#[async_trait]
pub trait AuthRepo: Send + Sync {
    async fn issue(&self, user_id: &UserId) -> Result<String, TokenRepoError>;
    async fn verify(&self, token: &str)     -> Result<bool, TokenRepoError>;
}

// ── Errors ───────────────────────────────────────────────────────────────

#[derive(Debug, thiserror::Error)]
pub enum TokenRepoError {
    #[error("unauthorized")]
    Unauthorized,
}

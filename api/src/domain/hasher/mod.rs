use async_trait::async_trait;
use crate::domain::user::value_objects::{password_hash::PasswordHash, password_raw::PasswordRaw};

#[async_trait]
pub trait HasherService: Send + Sync {
    async fn hash(&self, raw: &PasswordRaw)                        -> Result<PasswordHash, HasherServiceError>;
    async fn verify(&self, raw: &PasswordRaw, hash: &PasswordHash) -> Result<bool, HasherServiceError>;
}

// ── Errors ───────────────────────────────────────────────────────────────

#[derive(Debug, thiserror::Error)]
pub enum HasherServiceError {
    #[error("hasher error: {0}")]
    Hash(String),
    #[error("tokio task error: {0}")]
    TokioTask(String),
}

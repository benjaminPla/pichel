use argon2::password_hash;
use async_trait::async_trait;
use crate::domain::user::value_objects::{password_hash::PasswordHash, password_raw::PasswordRaw};

#[async_trait]
pub trait Hasher: Send + Sync + 'static {
    async fn hash(&self, raw: &PasswordRaw) -> Result<PasswordHash, HasherError>;
    async fn verify(&self, raw: &PasswordRaw, hash: &PasswordHash)
        -> Result<bool, HasherError>;
}

// ── Errors ───────────────────────────────────────────────────────────────

#[derive(Debug, thiserror::Error)]
pub enum HasherError {
    #[error("hash failed: {0}")]
    Hash(String),
    #[error("tokio task {0}")]
    TokioTask(String),
}

impl From<password_hash::Error> for HasherError {
    fn from(e: password_hash::Error) -> Self {
        HasherError::Hash(e.to_string())
    }
}

impl From<tokio::task::JoinError> for HasherError {
    fn from(e: tokio::task::JoinError) -> Self {
        HasherError::TokioTask(e.to_string())
    }
}

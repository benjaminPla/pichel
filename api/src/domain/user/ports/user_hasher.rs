use async_trait::async_trait;

use crate::domain::user::value_objects::{password_hash::PasswordHash, password_raw::PasswordRaw};

#[async_trait]
pub trait UserHasher: Send + Sync + 'static {
    async fn hash(&self, raw: &PasswordRaw) -> Result<PasswordHash, UserHasherError>;
    async fn verify(&self, raw: &PasswordRaw, hash: &PasswordHash)
        -> Result<bool, UserHasherError>;
}

// ── Errors ───────────────────────────────────────────────────────────────

#[derive(Debug, thiserror::Error)]
pub enum UserHasherError {
    #[error("hash failed")]
    Hash,
    #[error("verify failed")]
    Verify,
}

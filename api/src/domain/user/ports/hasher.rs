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
    #[error("hash failed")]
    Hash,
    #[error("verify failed")]
    Verify,
}

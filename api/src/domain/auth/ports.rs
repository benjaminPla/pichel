use async_trait::async_trait;
use crate::domain::user::value_objects::id::UserId;

#[async_trait]
pub trait TokenIssuer: Send + Sync {
    async fn issue(&self, user_id: &UserId) -> Result<String, TokenIssuerError>;
    async fn verify(&self, token: &str)     -> Result<bool, TokenIssuerError>;
}

// ── Errors ───────────────────────────────────────────────────────────────

#[derive(Debug, thiserror::Error)]
pub enum TokenIssuerError {
    #[error("unauthorized")]
    Issue,
    #[error("unauthorized")]
    Verify,
}

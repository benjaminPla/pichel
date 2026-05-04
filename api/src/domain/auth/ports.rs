use crate::domain::{auth::Claims, user::value_objects::id::UserId};

pub trait TokenService: Send + Sync {
    fn issue(&self, user_id: &UserId) -> Result<String, TokenServiceError>;
    fn validate(&self, token: &str)   -> Result<Claims, TokenServiceError>;
}

// ── Errors ───────────────────────────────────────────────────────────────

#[derive(Debug, thiserror::Error)]
pub enum TokenServiceError {
    #[error("token error: {0}")]
    Internal(String),
}

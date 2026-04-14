use super::user::{AuthDomainError, UserId};

/// PORT — the domain declares it needs token capabilities,
/// but stays ignorant of JWT, PASETO, or any token technology.
/// Infrastructure provides the adapter (JwtTokenService).
pub trait TokenService: Send + Sync {
    fn issue(&self, user_id: &UserId) -> Result<String, AuthDomainError>;
    fn validate(&self, token: &str) -> Result<UserId, AuthDomainError>;
}

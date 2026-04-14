use async_trait::async_trait;
use super::user::{AuthDomainError, Email, User};

/// PORT — the domain declares it needs persistence,
/// but stays ignorant of Postgres, Redis, or any storage tech.
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn save(&self, user: &User) -> Result<(), AuthDomainError>;
    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, AuthDomainError>;
    async fn exists_by_email(&self, email: &Email) -> Result<bool, AuthDomainError>;
}

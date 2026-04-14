use thiserror::Error;
use uuid::Uuid;

// ── Value Objects ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserId(pub Uuid);

#[derive(Debug, Clone)]
pub struct Email(String);

impl Email {
    pub fn new(value: impl Into<String>) -> Result<Self, AuthDomainError> {
        let s = value.into();
        if s.contains('@') && s.len() > 3 {
            Ok(Self(s))
        } else {
            Err(AuthDomainError::InvalidEmail)
        }
    }
    pub fn value(&self) -> &str { &self.0 }
}

#[derive(Debug, Clone)]
pub struct HashedPassword(String);

impl HashedPassword {
    /// Hash a raw password. CPU-intensive — wrap in `spawn_blocking` in prod.
    pub fn from_raw(raw: &str) -> Result<Self, AuthDomainError> {
        use argon2::{
            password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
            Argon2,
        };
        let salt = SaltString::generate(&mut OsRng);
        let hash = Argon2::default()
            .hash_password(raw.as_bytes(), &salt)
            .map_err(|_| AuthDomainError::HashingFailed)?
            .to_string();
        Ok(Self(hash))
    }

    pub fn verify(&self, raw: &str) -> bool {
        use argon2::{
            password_hash::{PasswordHash, PasswordVerifier},
            Argon2,
        };
        PasswordHash::new(&self.0)
            .ok()
            .map(|h| Argon2::default().verify_password(raw.as_bytes(), &h).is_ok())
            .unwrap_or(false)
    }

    /// Reconstruct from a stored hash (no re-hashing).
    pub fn from_hash(hash: String) -> Self { Self(hash) }
    pub fn value(&self) -> &str { &self.0 }
}

// ── Aggregate Root ────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub email: Email,
    pub password: HashedPassword,
}

impl User {
    pub fn register(email: Email, password: HashedPassword) -> Self {
        Self { id: UserId(Uuid::new_v4()), email, password }
    }
}

// ── Errors ────────────────────────────────────────────────────────────────────

#[derive(Debug, Error)]
pub enum AuthDomainError {
    #[error("invalid email format")]
    InvalidEmail,
    #[error("email already registered")]
    EmailAlreadyTaken,
    #[error("invalid credentials")]
    InvalidPassword,
    #[error("password hashing failed")]
    HashingFailed,
    #[error("token error: {0}")]
    Token(String),
    #[error("internal error")]
    Internal,
}

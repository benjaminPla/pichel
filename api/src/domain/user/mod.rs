pub mod ports;
pub mod value_objects;

use crate::domain::shared::value_objects::email::Email;
use value_objects::{id::UserId, password_hash::PasswordHash};

pub struct User {
    email:         Email,
    id:            UserId,
    password_hash: PasswordHash,
}

impl User {
    pub fn new(email: Email, password_hash: PasswordHash) -> Self {
        let id = UserId::new();
        Self { email, id, password_hash }
    }

    pub fn reconstitute(email: Email, id: UserId) -> Self {
        Self { email, id, password_hash: PasswordHash::new("") }
    }

    pub fn reconstitute_with_password_hash(email: Email, id: UserId, password_hash: PasswordHash) -> Self {
        Self { email, id, password_hash }
    }

    // ── Getters ──────────────────────────────────────────────────────────────

    pub fn get_id(&self)            -> &UserId       { &self.id }
    pub fn get_email(&self)         -> &Email        { &self.email }
    pub fn get_password_hash(&self) -> &PasswordHash { &self.password_hash }
} 

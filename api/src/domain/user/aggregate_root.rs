use super::value_objects::{email::Email, id::UserId};

pub struct User {
    email: Email,
    id: UserId,
    password_hash: String,
}

impl User {
    pub fn new(email: Email, password_hash: String) -> Self {
        let id = UserId::new();
        Self { email, id, password_hash }
    }

    pub fn reconstitute(email: Email, id: UserId, password_hash: String) -> Self {
        Self { email, id, password_hash }
    }

    // ── Getters ──────────────────────────────────────────────────────────────

    pub fn get_id(&self) -> &UserId { &self.id }
    pub fn get_email(&self) -> &Email { &self.email }
} 

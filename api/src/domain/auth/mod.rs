pub mod ports;

use chrono::Utc;
use crate::domain::user::value_objects::id::UserId;

#[derive(Clone)]
pub struct Claims {
    exp: i64,
    sub: UserId,
}

impl Claims {
    const EXP_24_HR: i64 = 60 * 60 * 24;

    pub fn new(sub: UserId) -> Self {
        let now = Utc::now().timestamp();
        Self { exp: now + Self::EXP_24_HR, sub }
    }

    pub fn reconstitute(exp: i64,  sub: UserId) -> Self {
        Self { exp, sub }
    }

    // ── Getters ──────────────────────────────────────────────────────────────

    pub fn get_exp(&self) -> i64     { self.exp }
    pub fn get_sub(&self) -> &UserId { &self.sub }
}

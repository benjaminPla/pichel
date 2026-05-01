use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::user::aggregate_root::User;

// ── UserCreate ───────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct UserCreateRequestBody {
    pub email:    String,
    pub password: String,
}

#[derive(Serialize)]
pub struct UserCreateResponse {
    email: String,
    id:    Uuid,
}

impl From<User> for UserCreateResponse {
    fn from(u: User) -> Self {
        Self {
            email: u.get_email().value().to_string(),
            id:    u.get_id().value(),
        }
    }
}

// ── UserGetAll ───────────────────────────────────────────────────────────

#[derive(Serialize)]
pub struct UserGetAllItem {
    email: String,
    id:    Uuid,
}

impl From<User> for UserGetAllItem {
    fn from(u: User) -> Self {
        Self {
            email: u.get_email().value().to_string(),
            id:    u.get_id().value(),
        }
    }
}

#[derive(Serialize)]
pub struct UserGetAllResponse {
    pub users: Vec<UserGetAllItem>,
    pub total: i64,
}

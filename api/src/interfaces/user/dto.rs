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

// ── UserUpdate ───────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct UserUpdateRequestBody {
    pub email: Option<String>,
}

#[derive(Serialize)]
pub struct UserUpdateResponse {
    email: String,
    id:    Uuid,
}

impl From<User> for UserUpdateResponse {
    fn from(u: User) -> Self {
        Self {
            email: u.get_email().value().to_string(),
            id:    u.get_id().value(),
        }
    }
}

// ── UserUpdatePassword ───────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct UserUpdatePasswordRequestBody {
    pub password: String,
}

#[derive(Serialize)]
pub struct UserUpdatePasswordResponse {
    email: String,
    id:    Uuid,
}

impl From<User> for UserUpdatePasswordResponse {
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
    pub total: i64,
    pub users: Vec<UserGetAllItem>,
}

// ── UserGetById ──────────────────────────────────────────────────────────

#[derive(Serialize)]
pub struct UserGetByIdResponse {
    pub email: String,
    pub id:    Uuid,
}

impl From<User> for UserGetByIdResponse {
    fn from(u: User) -> Self {
        Self {
            email: u.get_email().value().to_string(),
            id:    u.get_id().value(),
        }
    }
}

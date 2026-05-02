use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::user::User;

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

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::user::User;

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

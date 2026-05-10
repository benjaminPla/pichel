use serde::Serialize;
use uuid::Uuid;
use crate::domain::user::User;

#[derive(Serialize)]
pub struct UserDeleteResponse {
    id:    Uuid,
    email: String,
}

impl From<User> for UserDeleteResponse {
    fn from(u: User) -> Self {
        Self {
            id:    u.get_id().value(),
            email: u.get_email().value().to_string(),
        }
    }
}

use serde::Serialize;
use uuid::Uuid;
use crate::domain::user::User;

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

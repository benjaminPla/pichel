use crate::domain::user::User;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

#[derive(Deserialize)]
pub struct GetAllQueryParams {
    pub page:     i64,
    pub per_page: i64
}

impl GetAllQueryParams {
    pub const MAX_PER_PAGE: i64 = 50;
}

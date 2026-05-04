use crate::{
    application::user::errors::UserAppError,
    domain::user::{ports::UserRepo, value_objects::email::Email, User},
};
use std::sync::Arc;

// ── Query ────────────────────────────────────────────────────────────────

pub struct UserGetByEmailQuery {
    pub email: String,
}

// ── Handler ──────────────────────────────────────────────────────────────

pub struct UserGetByEmailHandler {
    user_repo: Arc<dyn UserRepo>,
}

impl UserGetByEmailHandler {
    pub fn new(user_repo: Arc<dyn UserRepo>) -> Self {
        Self { user_repo }
    }

    pub async fn execute(&self, query: UserGetByEmailQuery) -> Result<User, UserAppError> {
        let email = Email::new(query.email)?;
        let user  = self.user_repo.get_by_email(&email).await?;
        Ok(user)
    }
}

use crate::{
    application::user::errors::UserAppError,
    domain::user::{ports::repository::UserRepo, value_objects::id::UserId, User},
};
use std::sync::Arc;
use uuid::Uuid;

// ── Query ────────────────────────────────────────────────────────────────

pub struct UserGetByIdQuery {
    pub id: Uuid,
}

// ── Handler ──────────────────────────────────────────────────────────────

pub struct UserGetByIdHandler {
    user_repo: Arc<dyn UserRepo>,
}

impl UserGetByIdHandler {
    pub fn new(user_repo: Arc<dyn UserRepo>) -> Self {
        Self { user_repo }
    }

    pub async fn execute(&self, query: UserGetByIdQuery) -> Result<User, UserAppError> {
        let id   = UserId::reconstitute(query.id);
        let user = self.user_repo.get_by_id(&id).await?;
        Ok(user)
    }
}

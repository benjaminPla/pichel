use crate::{
    application::user::errors::UserAppError,
    domain::user::{ports::UserRepo, value_objects::id::UserId, User},
};
use std::sync::Arc;
use uuid::Uuid;

pub struct GetUserByIdInput {
    pub id: Uuid,
}

pub struct GetUserByIdUseCase {
    user_repo: Arc<dyn UserRepo>,
}

impl GetUserByIdUseCase {
    pub fn new(user_repo: Arc<dyn UserRepo>) -> Self {
        Self { user_repo }
    }

    pub async fn execute(&self, input: GetUserByIdInput) -> Result<User, UserAppError> {
        let id   = UserId::reconstitute(input.id);
        let user = self.user_repo.get_by_id(&id).await?;
        Ok(user)
    }
}

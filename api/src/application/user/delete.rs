use crate::{
    application::user::errors::UserAppError,
    domain::user::{ports::UserRepo, value_objects::id::UserId, User},
};
use std::sync::Arc;
use uuid::Uuid;

pub struct DeleteUserInput {
    pub id: Uuid,
}

pub struct DeleteUserUseCase {
    user_repo: Arc<dyn UserRepo>,
}

impl DeleteUserUseCase {
    pub fn new(user_repo: Arc<dyn UserRepo>) -> Self {
        Self { user_repo }
    }

    pub async fn execute(&self, input: DeleteUserInput) -> Result<User, UserAppError> {
        let user_id = UserId::reconstitute(input.id);
        let user    = self.user_repo.delete(&user_id).await?;
        Ok(user)
    }
}

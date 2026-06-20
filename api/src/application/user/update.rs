use crate::{
    application::user::errors::UserAppError,
    domain::{
        shared::value_objects::email::Email,
        user::{
            ports::UserRepo,
            value_objects::id::UserId,
            User,
        },
    },
};
use std::sync::Arc;
use uuid::Uuid;

pub struct UpdateUserInput {
    pub email: Option<String>,
    pub id:    Uuid,
}

pub struct UpdateUserUseCase {
    user_repo: Arc<dyn UserRepo>,
}

impl UpdateUserUseCase {
    pub fn new(user_repo: Arc<dyn UserRepo>) -> Self {
        Self { user_repo }
    }

    pub async fn execute(&self, input: UpdateUserInput) -> Result<User, UserAppError> {
        let user_id = UserId::reconstitute(input.id);
        let current = self.user_repo.get_by_id(&user_id).await?;
        let email = match input.email {
            Some(e) => Email::new(e)?,
            None    => current.get_email().clone(),
        };
        let updated = User::reconstitute(email, current.get_id().clone());
        let user    = self.user_repo.update(&updated).await?;
        Ok(user)
    }
}

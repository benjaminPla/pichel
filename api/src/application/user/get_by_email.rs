use crate::{
    application::user::errors::UserAppError,
    domain::user::{ports::UserRepo, value_objects::email::Email, User},
};
use std::sync::Arc;

pub struct GetUserByEmailInput {
    pub email: String,
}

pub struct GetUserByEmailUseCase {
    user_repo: Arc<dyn UserRepo>,
}

impl GetUserByEmailUseCase {
    pub fn new(user_repo: Arc<dyn UserRepo>) -> Self {
        Self { user_repo }
    }

    pub async fn execute(&self, input: GetUserByEmailInput) -> Result<User, UserAppError> {
        let email = Email::new(input.email)?;
        let user  = self.user_repo.get_by_email(&email).await?;
        Ok(user)
    }
}

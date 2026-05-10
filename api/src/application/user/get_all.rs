use crate::{
    application::user::errors::UserAppError,
    domain::user::{ports::UserRepo, User},
};
use std::sync::Arc;

pub struct GetAllUsersInput {
    pub page:     i64,
    pub per_page: i64,
}

pub struct GetAllUsersUseCase {
    user_repo: Arc<dyn UserRepo>,
}

impl GetAllUsersUseCase {
    pub fn new(user_repo: Arc<dyn UserRepo>) -> Self {
        Self { user_repo }
    }

    pub async fn execute(&self, input: GetAllUsersInput) -> Result<(Vec<User>, i64), UserAppError> {
        let (users, total) = self.user_repo.get_all(input.page, input.per_page).await?;
        Ok((users, total))
    }
}

use std::sync::Arc;

use crate::{
    application::user::errors::UserAppError,
    domain::user::{aggregate_root::User, repository::UserRepo},
};

struct UserCreateCommand {
    email:    String,
    password: String,
}

struct UserCreateHandler {
    user_repo: Arc<dyn UserRepo>,
}

impl UserCreateHandler {
    pub fn new(user_repo: Arc<dyn UserRepo>) -> Self {
        Self { user_repo }
    }

    pub async fn execute(&self, cmd: UserCreateCommand) -> Result<User, UserAppError> {
        let email         = Email::new(cmd.email)?;
        let password      = PasswordRaw::new(cmd.password)?;
        let password_hash = PasswordHash::hash(password)?;
        let user          = User::new(email, password_hash);
        let user          = self.user_repo.save(&user).await?;
        Ok(user)
    }
}

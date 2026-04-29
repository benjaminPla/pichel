use std::sync::Arc;

use crate::{
    application::user::errors::UserAppError,
    domain::user::{
        aggregate_root::User,
        ports::user_hasher::UserHasher,
        repository::UserRepo,
        value_objects::{email::Email, password_raw::PasswordRaw},
    },
};

pub struct UserCreateCommand {
    pub email:    String,
    pub password: String,
}

pub struct UserCreateHandler {
    hasher:    Arc<dyn UserHasher>,
    user_repo: Arc<dyn UserRepo>,
}

impl UserCreateHandler {
    pub fn new(hasher: Arc<dyn UserHasher>, user_repo: Arc<dyn UserRepo>) -> Self {
        Self { hasher, user_repo }
    }

    pub async fn execute(&self, cmd: UserCreateCommand) -> Result<User, UserAppError> {
        let email         = Email::new(cmd.email)?;
        let password      = PasswordRaw::new(cmd.password)?;
        let password_hash = self.hasher.hash(&password).await?;
        let user          = User::new(email, password_hash);
        let user          = self.user_repo.save(&user).await?;
        Ok(user)
    }
}

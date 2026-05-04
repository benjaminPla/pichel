use crate::{
    application::user::errors::UserAppError,
    domain::{
        hasher::HasherService,
        user::{
            ports::UserRepo,
            value_objects::{email::Email, password_raw::PasswordRaw},
            User,
        },
    },
};
use std::sync::Arc;

// ── Command ──────────────────────────────────────────────────────────────

pub struct UserCreateCommand {
    pub email:    String,
    pub password: String,
}

// ── Handler ──────────────────────────────────────────────────────────────

pub struct UserCreateHandler {
    hasher_service: Arc<dyn HasherService>,
    user_repo:      Arc<dyn UserRepo>,
}

impl UserCreateHandler {
    pub fn new(hasher_service: Arc<dyn HasherService>, user_repo: Arc<dyn UserRepo>) -> Self {
        Self { hasher_service, user_repo }
    }

    pub async fn execute(&self, cmd: UserCreateCommand) -> Result<User, UserAppError> {
        let email         = Email::new(cmd.email)?;
        let password      = PasswordRaw::new(cmd.password)?;
        let password_hash = self.hasher_service.hash(&password).await?;
        let user          = User::new(email, password_hash);
        let user          = self.user_repo.create(&user).await?;
        Ok(user)
    }
}

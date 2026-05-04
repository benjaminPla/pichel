use crate::{
    application::user::errors::UserAppError,
    domain::{
        hasher::HasherService,
        user::{
            ports::UserRepo,
            value_objects::{id::UserId, password_raw::PasswordRaw},
            User,
        },
    },
};
use std::sync::Arc;
use uuid::Uuid;

// ── Command ──────────────────────────────────────────────────────────────

pub struct UserUpdatePasswordCommand {
    pub id:       Uuid,
    pub password: String,
}

// ── Handler ──────────────────────────────────────────────────────────────

pub struct UserUpdatePasswordHandler {
    hasher_service: Arc<dyn HasherService>,
    user_repo:      Arc<dyn UserRepo>,
}

impl UserUpdatePasswordHandler {
    pub fn new(hasher_service: Arc<dyn HasherService>, user_repo: Arc<dyn UserRepo>) -> Self {
        Self { hasher_service, user_repo }
    }

    pub async fn execute(&self, cmd: UserUpdatePasswordCommand) -> Result<User, UserAppError> {
        let user_id       = UserId::reconstitute(cmd.id);
        self.user_repo.get_by_id(&user_id).await?;
        let raw           = PasswordRaw::new(cmd.password)?;
        let password_hash = self.hasher_service.hash(&raw).await?;
        let user          = self.user_repo.update_password(&user_id, &password_hash).await?;
        Ok(user)
    }
}

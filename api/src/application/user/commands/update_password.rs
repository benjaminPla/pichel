use crate::{
    application::user::errors::UserAppError,
    domain::user::{
        ports::hasher::Hasher,
        ports::repository::UserRepo,
        value_objects::{id::UserId, password_raw::PasswordRaw},
        User,
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
    hasher:    Arc<dyn Hasher>,
    user_repo: Arc<dyn UserRepo>,
}

impl UserUpdatePasswordHandler {
    pub fn new(hasher: Arc<dyn Hasher>, user_repo: Arc<dyn UserRepo>) -> Self {
        Self { hasher, user_repo }
    }

    pub async fn execute(&self, cmd: UserUpdatePasswordCommand) -> Result<User, UserAppError> {
        let user_id       = UserId::reconstitute(cmd.id);
        self.user_repo.get_by_id(&user_id).await?;
        let raw           = PasswordRaw::new(cmd.password)?;
        let password_hash = self.hasher.hash(&raw).await?;
        let user          = self.user_repo.update_password(&user_id, &password_hash).await?;
        Ok(user)
    }
}

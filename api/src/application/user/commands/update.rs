use crate::{
    application::user::errors::UserAppError,
    domain::user::{
        ports::UserRepo,
        value_objects::{email::Email, id::UserId},
        User,
    },
};
use std::sync::Arc;
use uuid::Uuid;

// ── Command ──────────────────────────────────────────────────────────────

pub struct UserUpdateCommand {
    pub email: Option<String>,
    pub id:    Uuid,
}

// ── Handler ──────────────────────────────────────────────────────────────

pub struct UserUpdateHandler {
    user_repo: Arc<dyn UserRepo>,
}

impl UserUpdateHandler {
    pub fn new(user_repo: Arc<dyn UserRepo>) -> Self {
        Self { user_repo }
    }

    pub async fn execute(&self, cmd: UserUpdateCommand) -> Result<User, UserAppError> {
        let user_id = UserId::reconstitute(cmd.id);
        let current = self.user_repo.get_by_id(&user_id).await?;
        let email = match cmd.email {
            Some(e) => Email::new(e)?,
            None    => current.get_email().clone(),
        };
        let updated = User::reconstitute(email, current.get_id().clone(), current.get_password_hash().clone());
        let user    = self.user_repo.update(&updated).await?;
        Ok(user)
    }
}

use crate::{
    application::auth::errors::AuthAppError,
    domain::{
        auth::ports::TokenService,
        hasher::HasherService,
        user::{
            ports::UserRepo,
            value_objects::{email::Email, password_raw::PasswordRaw},
        },
    },
};
use std::sync::Arc;

pub struct AuthenticateInput {
    pub email:    String,
    pub password: String,
}

pub struct AuthenticateUseCase {
    hasher_service: Arc<dyn HasherService>,
    token_service:  Arc<dyn TokenService>,
    user_repo:      Arc<dyn UserRepo>,
}

impl AuthenticateUseCase {
    pub fn new(
        hasher_service: Arc<dyn HasherService>,
        token_service:  Arc<dyn TokenService>,
        user_repo:      Arc<dyn UserRepo>,
    ) -> Self {
        Self { hasher_service, token_service, user_repo }
    }

    pub async fn execute(&self, input: AuthenticateInput) -> Result<String, AuthAppError> {
        let email    = Email::new(input.email)?;
        let user     = self.user_repo.get_by_email(&email).await?;
        let password = PasswordRaw::new(input.password)?;
        let valid    = self.hasher_service.verify(&password, user.get_password_hash()).await?;
        if !valid { return Err(AuthAppError::Unauthorized); }
        let token    = self.token_service.issue(user.get_id())?;
        Ok(token)
    }
}

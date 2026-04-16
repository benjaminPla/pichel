use std::sync::Arc;
use crate::domain::auth::{AuthDomainError, Email, TokenService, UserRepository};

pub struct LoginUseCase {
    user_repo: Arc<dyn UserRepository>,
    token_service: Arc<dyn TokenService>,
}

impl LoginUseCase {
    pub fn new(user_repo: Arc<dyn UserRepository>, token_service: Arc<dyn TokenService>) -> Self {
        Self { user_repo, token_service }
    }

    pub async fn execute(&self, email: String, password: String) -> Result<String, AuthDomainError> {
        let email = Email::new(email)?;

        let user = self.user_repo
            .find_by_email(&email)
            .await?
            .ok_or(AuthDomainError::InvalidPassword)?;

        if !user.password.verify(&password) {
            return Err(AuthDomainError::InvalidPassword);
        }

        self.token_service.issue(&user.id)
    }
}

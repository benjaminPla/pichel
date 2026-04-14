use std::sync::Arc;
use crate::domain::auth::{
    AuthDomainError, Email, HashedPassword, TokenService, User, UserRepository,
};

pub struct RegisterUseCase {
    user_repo: Arc<dyn UserRepository>,
    token_service: Arc<dyn TokenService>,
}

impl RegisterUseCase {
    pub fn new(user_repo: Arc<dyn UserRepository>, token_service: Arc<dyn TokenService>) -> Self {
        Self { user_repo, token_service }
    }

    pub async fn execute(&self, email: String, password: String) -> Result<String, AuthDomainError> {
        let email = Email::new(email)?;

        if self.user_repo.exists_by_email(&email).await? {
            return Err(AuthDomainError::EmailAlreadyTaken);
        }

        // NOTE: argon2 is CPU-intensive — wrap in spawn_blocking in production
        let hashed = HashedPassword::from_raw(&password)?;
        let user = User::register(email, hashed);

        self.user_repo.save(&user).await?;

        self.token_service.issue(&user.id)
    }
}

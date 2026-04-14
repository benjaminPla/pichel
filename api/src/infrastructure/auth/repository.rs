use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::domain::auth::{
    repository::UserRepository, AuthDomainError, Email, HashedPassword, User, UserId,
};

pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self { Self { pool } }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn save(&self, user: &User) -> Result<(), AuthDomainError> {
        sqlx::query("INSERT INTO users (id, email, password_hash) VALUES ($1, $2, $3)")
            .bind(user.id.0)
            .bind(user.email.value())
            .bind(user.password.value())
            .execute(&self.pool)
            .await
            .map_err(|_| AuthDomainError::Internal)?;
        Ok(())
    }

    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, AuthDomainError> {
        let row = sqlx::query("SELECT id, email, password_hash FROM users WHERE email = $1")
            .bind(email.value())
            .fetch_optional(&self.pool)
            .await
            .map_err(|_| AuthDomainError::Internal)?;

        row.map(|r| {
            Ok(User {
                id: UserId(r.try_get::<Uuid, _>("id").map_err(|_| AuthDomainError::Internal)?),
                email: Email::new(
                    r.try_get::<String, _>("email").map_err(|_| AuthDomainError::Internal)?,
                )?,
                password: HashedPassword::from_hash(
                    r.try_get::<String, _>("password_hash").map_err(|_| AuthDomainError::Internal)?,
                ),
            })
        })
        .transpose()
    }

    async fn exists_by_email(&self, email: &Email) -> Result<bool, AuthDomainError> {
        let row =
            sqlx::query("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1) AS \"exists\"")
                .bind(email.value())
                .fetch_one(&self.pool)
                .await
                .map_err(|_| AuthDomainError::Internal)?;

        row.try_get::<bool, _>("exists").map_err(|_| AuthDomainError::Internal)
    }
}

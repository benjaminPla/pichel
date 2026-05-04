mod create;
mod errors;
mod get_all;
mod get_by_id;
mod row;
mod update;
mod update_password;

use async_trait::async_trait;
use crate::domain::user::{
    ports::{UserRepo, UserRepoError},
    value_objects::{id::UserId, password_hash::PasswordHash},
    User,
};
use sqlx::PgPool;

pub struct PgUserRepo {
    pool: PgPool,
}

impl PgUserRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepo for PgUserRepo {
    async fn create(&self, user: &User) -> Result<User, UserRepoError> {
        create::create(&self.pool, user).await
    }

    async fn get_all(&self, page: i64, per_page: i64) -> Result<(Vec<User>, i64), UserRepoError> {
        get_all::get_all(&self.pool, page, per_page).await
    }

    async fn get_by_id(&self, user_id: &UserId) -> Result<User, UserRepoError> {
        get_by_id::get_by_id(&self.pool, user_id).await
    }

    async fn update(&self, user: &User) -> Result<User, UserRepoError> {
        update::update(&self.pool, user).await
    }

    async fn update_password(
        &self,
        user_id: &UserId,
        password_hash: &PasswordHash,
    ) -> Result<User, UserRepoError> {
        update_password::update_password(&self.pool, user_id, password_hash).await
    }
}

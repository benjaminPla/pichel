use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::user::{
    aggregate_root::User,
    repository::{UserRepo, UserRepoError},
    value_objects::id::UserId,
};

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
    async fn get_all(&self, page: i64, per_page: i64) -> Result<(Vec<User>, i64), UserRepoError> {
        todo!()
    }

    async fn get_by_id(&self, user_id: &UserId) -> Result<User, UserRepoError> {
        todo!()
    }

    async fn save(&self, user: &User) -> Result<User, UserRepoError> {
        todo!()
    }
}

use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::user::{
    aggregate_root::User, ports::repository::UserRepo, ports::repository::UserRepoError,
    value_objects::id::UserId,
};

use super::row::UserRow;

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
        let offset     = (page - 1) * per_page;
        let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users").fetch_one(&self.pool).await.map_err(|_| UserRepoError::Database)?;
        let rows       = sqlx::query_as::<_, UserRow>(
            "SELECT email, id, password_hash
             FROM users
             ORDER BY name ASC
             LIMIT $1 OFFSET $2"
        )
        .bind(per_page)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|_| UserRepoError::Database)?;
        let users = rows.into_iter().map(User::try_from).collect::<Result<Vec<_>, _>>();
        let users = users.map_err(|e| UserRepoError::Mapping(e.to_string()))?;
        Ok((users, total))
    }

    async fn get_by_id(&self, user_id: &UserId) -> Result<User, UserRepoError> {
        let row = sqlx::query_as::<_, UserRow>(
            "SELECT email, id, password_hash
            FROM users
            WHERE id = $1"
        )
        .bind(user_id.value())
        .fetch_one(&self.pool)
        .await
        .map_err(|_| UserRepoError::Database)?;
        let user = User::try_from(row)?;
        Ok(user)
    }

    async fn save(&self, user: &User) -> Result<User, UserRepoError> {
        let row = sqlx::query_as::<_, UserRow>(
            "INSERT INTO users
               (email, id, password_hash)
             VALUES ($1,$2,$3)
             ON CONFLICT (id) DO UPDATE SET
                email         = EXCLUDED.email,
                password_hash = EXCLUDED.password_hash
            RETURNING email, id, password_hash
            ",

        )
        .bind(&user.get_email().value())
        .bind(&user.get_id().value())
        .bind(&user.get_password_hash().value())
        .fetch_one(&self.pool)
        .await
        .map_err(|_| UserRepoError::Database)?;
        let user = User::try_from(row)?;
        Ok(user)
    }
}


use crate::domain::user::{ports::UserRepoError, value_objects::id::UserId};
use sqlx::PgPool;

pub async fn delete(pool: &PgPool, user_id: &UserId) -> Result<(), UserRepoError> {
    let result = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(user_id.value())
        .execute(pool)
        .await
        .map_err(|e| UserRepoError::Database(e.to_string()))?;
    if result.rows_affected() == 0 {
        return Err(UserRepoError::NotFound);
    }
    Ok(())
}

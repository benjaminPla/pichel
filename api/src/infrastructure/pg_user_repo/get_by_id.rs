use crate::domain::user::{ports::UserRepoError, value_objects::id::UserId, User};
use crate::infrastructure::pg_user_repo::row::UserRow;
use sqlx::PgPool;

pub async fn get_by_id(pool: &PgPool, user_id: &UserId) -> Result<User, UserRepoError> {
    let row = sqlx::query_as::<_, UserRow>(
        "SELECT email, id
         FROM users
         WHERE id = $1",
    )
    .bind(user_id.value())
    .fetch_one(pool)
    .await?;
    Ok(User::try_from(row)?)
}

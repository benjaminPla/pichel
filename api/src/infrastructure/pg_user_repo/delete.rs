use crate::{
    domain::user::{ports::UserRepoError, value_objects::id::UserId, User},
    infrastructure::pg_user_repo::row::UserRow,
};
use sqlx::PgPool;

pub async fn delete(pool: &PgPool, user_id: &UserId) -> Result<User, UserRepoError> {
    let row = sqlx::query_as::<_, UserRow>(
        "DELETE FROM users
         WHERE id = $1
         RETURNING email, id",
    )
    .bind(user_id.value())
    .fetch_one(pool)
    .await?;
    Ok(User::try_from(row)?)
}

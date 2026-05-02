use super::super::row::UserRow;
use crate::domain::user::{ports::repository::UserRepoError, value_objects::id::UserId, User};
use sqlx::PgPool;

pub(super) async fn get_by_id(pool: &PgPool, user_id: &UserId) -> Result<User, UserRepoError> {
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

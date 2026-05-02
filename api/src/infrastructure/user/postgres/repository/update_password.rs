use super::super::row::UserRow;
use crate::domain::user::{
    ports::repository::UserRepoError,
    value_objects::{id::UserId, password_hash::PasswordHash},
    User,
};
use sqlx::PgPool;

pub(super) async fn update_password(pool: &PgPool, user_id: &UserId, password_hash: &PasswordHash) -> Result<User, UserRepoError> {
    let row = sqlx::query_as::<_, UserRow>(
        "UPDATE users SET
           password_hash = $2
         WHERE id = $1
         RETURNING email, id",
    )
    .bind(&user_id.value())
    .bind(&password_hash.value())
    .fetch_one(pool)
    .await?;
    Ok(User::try_from(row)?)
}

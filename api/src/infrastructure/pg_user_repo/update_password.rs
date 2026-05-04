use crate::domain::user::{
    ports::UserRepoError,
    value_objects::{id::UserId, password_hash::PasswordHash},
    User,
};
use crate::infrastructure::pg_user_repo::row::UserRow;
use sqlx::PgPool;

pub async fn update_password(
    pool: &PgPool,
    user_id: &UserId,
    password_hash: &PasswordHash,
) -> Result<User, UserRepoError> {
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

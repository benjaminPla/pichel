use super::super::row::UserRow;
use crate::domain::user::{ports::repository::UserRepoError, User};
use sqlx::PgPool;

pub(super) async fn create(pool: &PgPool, user: &User) -> Result<User, UserRepoError> {
    let row = sqlx::query_as::<_, UserRow>(
        "INSERT INTO users
           (email, id, password_hash)
         VALUES ($1,$2,$3)
         RETURNING email, id",
    )
    .bind(&user.get_email().value())
    .bind(&user.get_id().value())
    .bind(&user.get_password_hash().value())
    .fetch_one(pool)
    .await?;
    Ok(User::try_from(row)?)
}

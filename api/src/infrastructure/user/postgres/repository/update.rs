use sqlx::PgPool;
use crate::domain::user::{aggregate_root::User, ports::repository::UserRepoError};
use super::super::row::UserRow;

pub(super) async fn update(pool: &PgPool, user: &User) -> Result<User, UserRepoError> {
    let row = sqlx::query_as::<_, UserRow>(
        "UPDATE users SET
           email = $2
         WHERE id = $1
         RETURNING email, id",
    )
    .bind(&user.get_id().value())
    .bind(&user.get_email().value())
    .fetch_one(pool)
    .await?;
    Ok(User::try_from(row)?)
}

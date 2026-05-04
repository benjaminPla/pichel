use crate::domain::user::{ports::UserRepoError, User};
use crate::infrastructure::pg_user_repo::row::UserRow;
use sqlx::PgPool;

pub async fn update(pool: &PgPool, user: &User) -> Result<User, UserRepoError> {
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

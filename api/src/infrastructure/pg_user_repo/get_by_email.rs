use crate::domain::user::{ports::UserRepoError, value_objects::email::Email, User};
use crate::infrastructure::pg_user_repo::row::UserRowWithPasswordHash;
use sqlx::PgPool;

pub async fn get_by_email(pool: &PgPool, email: &Email) -> Result<User, UserRepoError> {
    let row = sqlx::query_as::<_, UserRowWithPasswordHash>(
        "SELECT email, id, password_hash
         FROM users
         WHERE email = $1"
    )
    .bind(email.value())
    .fetch_one(pool)
    .await?;
    Ok(User::try_from(row)?)
}

use sqlx::PgPool;
use crate::domain::user::{aggregate_root::User, ports::repository::UserRepoError};
use super::super::row::UserRow;

pub(super) async fn get_all(pool: &PgPool, page: i64, per_page: i64) -> Result<(Vec<User>, i64), UserRepoError> {
    let offset     = (page - 1) * per_page;
    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(pool)
        .await?;
    let rows = sqlx::query_as::<_, UserRow>(
        "SELECT email, id
         FROM users
         ORDER BY email ASC
         LIMIT $1 OFFSET $2",
    )
    .bind(per_page)
    .bind(offset)
    .fetch_all(pool)
    .await?;
    let users = rows.into_iter().map(User::try_from).collect::<Result<Vec<_>, _>>()?;
    Ok((users, total))
}

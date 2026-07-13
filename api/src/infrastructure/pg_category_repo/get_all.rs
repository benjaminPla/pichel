use crate::{
    domain::category::{ports::repository::CategoryRepoError, Category},
    infrastructure::pg_category_repo::row::CategoryRow,
};
use sqlx::PgPool;

pub async fn get_all(pool: &PgPool) -> Result<Vec<Category>, CategoryRepoError> {
    let rows = sqlx::query_as::<_, CategoryRow>(
        "SELECT id, name FROM categories ORDER BY name ASC",
    )
    .fetch_all(pool)
    .await?;
    let categories = rows.into_iter().map(Category::try_from).collect::<Result<Vec<_>, _>>()?;
    Ok(categories)
}

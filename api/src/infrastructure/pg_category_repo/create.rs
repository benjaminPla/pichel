use crate::{
    domain::category::{ports::repository::CategoryRepoError, Category},
    infrastructure::pg_category_repo::row::CategoryRow,
};
use sqlx::PgPool;

pub async fn create(pool: &PgPool, category: &Category) -> Result<Category, CategoryRepoError> {
    let row = sqlx::query_as::<_, CategoryRow>(
        "INSERT INTO categories (id, name)
         VALUES ($1, $2)
         RETURNING id, name",
    )
    .bind(category.get_id().value())
    .bind(category.get_name().value())
    .fetch_one(pool)
    .await?;
    Ok(Category::try_from(row)?)
}

use crate::{
    domain::category::{ports::repository::CategoryRepoError, Category},
    infrastructure::pg_category_repo::row::CategoryRow,
};
use sqlx::PgPool;

pub async fn update(pool: &PgPool, category: &Category) -> Result<Category, CategoryRepoError> {
    let row = sqlx::query_as::<_, CategoryRow>(
        "UPDATE categories SET name = $2
         WHERE id = $1
         RETURNING id, name",
    )
    .bind(category.get_id().value())
    .bind(category.get_name().value())
    .fetch_one(pool)
    .await?;
    Ok(Category::try_from(row)?)
}

use crate::domain::category::{ports::repository::CategoryRepoError, value_objects::id::CategoryId};
use sqlx::PgPool;

pub async fn delete(pool: &PgPool, category_id: &CategoryId) -> Result<(), CategoryRepoError> {
    let result = sqlx::query("DELETE FROM categories WHERE id = $1")
        .bind(category_id.value())
        .execute(pool)
        .await
        .map_err(|e| CategoryRepoError::Database(e.to_string()))?;
    if result.rows_affected() == 0 {
        return Err(CategoryRepoError::NotFound);
    }
    Ok(())
}

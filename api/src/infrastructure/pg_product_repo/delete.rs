use crate::domain::product::{ports::repository::ProductRepoError, value_objects::id::ProductId};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn delete(pool: &PgPool, product_id: &ProductId, updated_by: Uuid) -> Result<(), ProductRepoError> {
    let result = sqlx::query("DELETE FROM products WHERE id = $1")
        .bind(product_id.value())
        .execute(pool)
        .await
        .map_err(|e| ProductRepoError::Database(e.to_string()))?;
    if result.rows_affected() == 0 {
        return Err(ProductRepoError::NotFound);
    }
    sqlx::query("UPDATE app_settings SET updated_by = $1 WHERE key = 'price_list_updated_at'")
        .bind(updated_by)
        .execute(pool)
        .await
        .map_err(|e| ProductRepoError::Database(e.to_string()))?;
    Ok(())
}

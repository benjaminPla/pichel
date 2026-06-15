use crate::{
    domain::product::{ports::repository::ProductRepoError, value_objects::id::ProductId, Product},
    infrastructure::pg_product_repo::row::ProductRow,
};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn delete(pool: &PgPool, product_id: &ProductId, updated_by: Uuid) -> Result<Product, ProductRepoError> {
    let row = sqlx::query_as::<_, ProductRow>(
        "DELETE FROM products
         WHERE id = $1
         RETURNING description, id, image_url, name, price_cents, sale_mode, symbols, unit_of_measure",
    )
    .bind(product_id.value())
    .fetch_one(pool)
    .await?;
    sqlx::query("UPDATE app_settings SET updated_by = $1 WHERE key = 'price_list_updated_at'")
        .bind(updated_by)
        .execute(pool)
        .await
        .map_err(|e| ProductRepoError::Database(e.to_string()))?;
    Ok(Product::try_from(row)?)
}

use crate::{
    domain::product::{ports::repository::ProductRepoError, Product},
    infrastructure::pg_product_repo::row::ProductRow,
};
use sqlx::PgPool;
use uuid::Uuid;

const PLU_ASSIGNMENT_LOCK_KEY: i64 = 69_420;
const PLU_MAX: i32 = 800;

pub async fn create(pool: &PgPool, product: &Product, updated_by: Uuid) -> Result<Product, ProductRepoError> {
    let mut tx = pool.begin().await.map_err(|e| ProductRepoError::Database(e.to_string()))?;

    sqlx::query("SELECT pg_advisory_xact_lock($1)")
        .bind(PLU_ASSIGNMENT_LOCK_KEY)
        .execute(&mut *tx)
        .await
        .map_err(|e| ProductRepoError::Database(e.to_string()))?;

    let next_plu: Option<i32> = sqlx::query_scalar(
        "SELECT MIN(n) FROM generate_series(1, $1) AS n
         WHERE n NOT IN (SELECT plu FROM products)",
    )
    .bind(PLU_MAX)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| ProductRepoError::Database(e.to_string()))?;

    let plu = next_plu.ok_or_else(|| {
        ProductRepoError::Validation(format!("no plu numbers available (all {PLU_MAX} slots in use)"))
    })?;

    let row = sqlx::query_as::<_, ProductRow>(
        "INSERT INTO products
           (description, id, active, image_url, name, plu, price_cents, sale_mode, symbols, unit_of_measure)
         VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)
         RETURNING description, id, active, image_url, name, plu, price_cents, sale_mode, symbols, unit_of_measure",
    )
    .bind(&product.get_description().map(|d| d.value()))
    .bind(&product.get_id().value())
    .bind(product.get_active())
    .bind(&product.get_image_url())
    .bind(&product.get_name().value())
    .bind(plu as i16)
    .bind(product.get_price_cents().value() as i64)
    .bind(product.get_sale_mode().as_str())
    .bind(&product.get_symbols().iter().map(|s| s.to_string()).collect::<Vec<String>>())
    .bind(&product.get_unit_of_measure().as_str())
    .fetch_one(&mut *tx)
    .await?;

    sqlx::query("UPDATE app_settings SET updated_by = $1 WHERE key = 'price_list_updated_at'")
        .bind(updated_by)
        .execute(&mut *tx)
        .await
        .map_err(|e| ProductRepoError::Database(e.to_string()))?;

    tx.commit().await.map_err(|e| ProductRepoError::Database(e.to_string()))?;

    Ok(Product::try_from(row)?)
}

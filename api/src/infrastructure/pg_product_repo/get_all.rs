use crate::{
    domain::product::{ports::repository::ProductRepoError, Product},
    infrastructure::pg_product_repo::row::ProductRow,
};
use chrono::{DateTime, Utc};
use sqlx::PgPool;

pub async fn get_all(pool: &PgPool, page: i64, per_page: i64) -> Result<(Vec<Product>, i64, Option<DateTime<Utc>>), ProductRepoError> {
    let offset     = (page - 1) * per_page;
    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM products")
        .fetch_one(pool)
        .await
        .map_err(|e| ProductRepoError::Database(e.to_string()))?;
    let rows = sqlx::query_as::<_, ProductRow>(
        "SELECT id, active, image_url, name, price_cents, sale_mode, symbols, unit_of_measure
         FROM products
         ORDER BY name ASC
         LIMIT $1 OFFSET $2",
    )
    .bind(per_page)
    .bind(offset)
    .fetch_all(pool)
    .await?;
    let price_list_updated_at: Option<DateTime<Utc>> = sqlx::query_scalar(
        "SELECT updated_at FROM app_settings WHERE key = 'price_list_updated_at'",
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| ProductRepoError::Database(e.to_string()))?;
    let products = rows.into_iter().map(Product::try_from).collect::<Result<Vec<_>, _>>()?;
    Ok((products, total, price_list_updated_at))
}

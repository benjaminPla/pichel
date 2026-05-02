use sqlx::PgPool;
use crate::domain::product::{aggregate_root::Product, ports::repository::ProductRepoError};
use super::super::row::ProductRow;

pub(super) async fn get_all(pool: &PgPool, page: i64, per_page: i64) -> Result<(Vec<Product>, i64), ProductRepoError> {
    let offset     = (page - 1) * per_page;
    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM products")
        .fetch_one(pool)
        .await
        .map_err(|e| ProductRepoError::Database(e.to_string()))?;
    let rows = sqlx::query_as::<_, ProductRow>(
        "SELECT description, id, image_url, low_stock_threshold, name, price_cents, stock, symbols, unit_of_measure
         FROM products
         ORDER BY name ASC
         LIMIT $1 OFFSET $2",
    )
    .bind(per_page)
    .bind(offset)
    .fetch_all(pool)
    .await?;
    let products = rows.into_iter().map(Product::try_from).collect::<Result<Vec<_>, _>>()?;
    Ok((products, total))
}

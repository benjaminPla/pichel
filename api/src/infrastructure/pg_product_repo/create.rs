use crate::{
    domain::product::{ports::repository::ProductRepoError, Product},
    infrastructure::pg_product_repo::row::ProductRow,
};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create(pool: &PgPool, product: &Product, updated_by: Uuid) -> Result<Product, ProductRepoError> {
    let row = sqlx::query_as::<_, ProductRow>(
        "INSERT INTO products
           (id, active, image_url, name, price_cents, sale_mode, symbols, unit_of_measure)
         VALUES ($1,$2,$3,$4,$5,$6,$7,$8)
         RETURNING id, active, image_url, name, price_cents, sale_mode, symbols, unit_of_measure",
    )
    .bind(&product.get_id().value())
    .bind(product.get_active())
    .bind(&product.get_image_url())
    .bind(&product.get_name().value())
    .bind(product.get_price_cents().value() as i64)
    .bind(product.get_sale_mode().as_str())
    .bind(&product.get_symbols().iter().map(|s| s.to_string()).collect::<Vec<String>>())
    .bind(&product.get_unit_of_measure().as_str())
    .fetch_one(pool)
    .await?;
    sqlx::query("UPDATE app_settings SET updated_by = $1 WHERE key = 'price_list_updated_at'")
        .bind(updated_by)
        .execute(pool)
        .await
        .map_err(|e| ProductRepoError::Database(e.to_string()))?;
    Ok(Product::try_from(row)?)
}

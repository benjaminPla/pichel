use crate::{
    domain::product::{ports::repository::ProductRepoError, Product},
    infrastructure::pg_product_repo::row::ProductRow,
};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn update(pool: &PgPool, product: &Product, updated_by: Uuid) -> Result<Product, ProductRepoError> {
    let row = sqlx::query_as::<_, ProductRow>(
        "UPDATE products SET
           description     = $2,
           active          = $3,
           image_url       = $4,
           name            = $5,
           price_cents     = $6,
           sale_mode       = $7,
           symbols         = $8,
           unit_of_measure = $9
         WHERE id = $1
         RETURNING description, id, active, image_url, name, price_cents, sale_mode, symbols, unit_of_measure",
    )
    .bind(&product.get_id().value())
    .bind(&product.get_description().map(|d| d.value()))
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

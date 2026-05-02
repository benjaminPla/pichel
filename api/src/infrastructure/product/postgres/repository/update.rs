use super::super::row::ProductRow;
use crate::domain::product::{ports::repository::ProductRepoError, Product};
use sqlx::PgPool;

pub(super) async fn update(pool: &PgPool, product: &Product) -> Result<Product, ProductRepoError> {
    let row = sqlx::query_as::<_, ProductRow>(
        "UPDATE products SET
           description         = $2,
           image_url           = $3,
           low_stock_threshold = $4,
           name                = $5,
           price_cents         = $6,
           stock               = $7,
           symbols             = $8,
           unit_of_measure     = $9
         WHERE id = $1
         RETURNING description, id, image_url, low_stock_threshold, name, price_cents, stock, symbols, unit_of_measure",
    )
    .bind(&product.get_id().value())
    .bind(&product.get_description().map(|d| d.value()))
    .bind(&product.get_image_url())
    .bind(product.get_low_stock_threshold() as i32)
    .bind(&product.get_name().value())
    .bind(product.get_price_cents() as i64)
    .bind(product.get_stock() as i32)
    .bind(&product.get_symbols().iter().map(|s| s.to_string()).collect::<Vec<String>>())
    .bind(&product.get_unit_of_measure().as_str())
    .fetch_one(pool)
    .await?;
    Ok(Product::try_from(row)?)
}

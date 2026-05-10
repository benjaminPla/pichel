use crate::{
    domain::product::{ports::repository::ProductRepoError, Product},
    infrastructure::pg_product_repo::row::ProductRow,
};
use sqlx::PgPool;

pub async fn create(pool: &PgPool, product: &Product) -> Result<Product, ProductRepoError> {
    let row = sqlx::query_as::<_, ProductRow>(
        "INSERT INTO products
           (description, id, image_url, name, price_cents, sale_mode, symbols, unit_of_measure)
         VALUES ($1,$2,$3,$4,$5,$6,$7,$8)
         RETURNING description, id, image_url, name, price_cents, sale_mode, symbols, unit_of_measure",
    )
    .bind(&product.get_description().map(|d| d.value()))
    .bind(&product.get_id().value())
    .bind(&product.get_image_url())
    .bind(&product.get_name().value())
    .bind(product.get_price_cents().value() as i64)
    .bind(product.get_sale_mode().as_str())
    .bind(&product.get_symbols().iter().map(|s| s.to_string()).collect::<Vec<String>>())
    .bind(&product.get_unit_of_measure().as_str())
    .fetch_one(pool)
    .await?;
    Ok(Product::try_from(row)?)
}

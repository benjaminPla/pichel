use super::super::row::ProductRow;
use crate::domain::product::{
    ports::repository::ProductRepoError, value_objects::id::ProductId, Product,
};
use sqlx::PgPool;

pub(super) async fn get_by_id(pool: &PgPool, product_id: &ProductId) -> Result<Product, ProductRepoError> {
    let row = sqlx::query_as::<_, ProductRow>(
        "SELECT description, id, image_url, low_stock_threshold, name, price_cents, stock, symbols, unit_of_measure
         FROM products
         WHERE id = $1",
    )
    .bind(product_id.value())
    .fetch_one(pool)
    .await?;
    Ok(Product::try_from(row)?)
}

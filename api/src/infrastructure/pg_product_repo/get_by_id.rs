use crate::{
    domain::product::{ports::repository::ProductRepoError, value_objects::id::ProductId, Product},
    infrastructure::pg_product_repo::row::ProductRow,
};
use sqlx::PgPool;

pub async fn get_by_id(pool: &PgPool, product_id: &ProductId) -> Result<Product, ProductRepoError> {
    let row = sqlx::query_as::<_, ProductRow>(
        "SELECT description, id, active, image_url, name, price_cents, sale_mode, symbols, unit_of_measure
         FROM products
         WHERE id = $1",
    )
    .bind(product_id.value())
    .fetch_one(pool)
    .await?;
    Ok(Product::try_from(row)?)
}

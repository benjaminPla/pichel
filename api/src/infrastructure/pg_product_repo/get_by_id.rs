use crate::{
    domain::product::{ports::repository::ProductRepoError, value_objects::id::ProductId, Product},
    infrastructure::pg_product_repo::row::ProductRow,
};
use sqlx::PgPool;

pub async fn get_by_id(pool: &PgPool, product_id: &ProductId) -> Result<Product, ProductRepoError> {
    let row = sqlx::query_as::<_, ProductRow>(
        "SELECT
           p.description, p.id, p.active, p.image_url, p.name, p.plu, p.price_cents, p.sale_mode, p.symbols, p.unit_of_measure,
           COALESCE(array_agg(c.id ORDER BY c.name)   FILTER (WHERE c.id IS NOT NULL), '{}') AS category_ids,
           COALESCE(array_agg(c.name ORDER BY c.name) FILTER (WHERE c.id IS NOT NULL), '{}') AS category_names
         FROM products p
         LEFT JOIN product_categories pc ON pc.product_id = p.id
         LEFT JOIN categories c          ON c.id = pc.category_id
         WHERE p.id = $1
         GROUP BY p.id",
    )
    .bind(product_id.value())
    .fetch_one(pool)
    .await?;
    Ok(Product::try_from(row)?)
}

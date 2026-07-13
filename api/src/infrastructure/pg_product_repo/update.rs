use crate::{
    domain::product::{ports::repository::ProductRepoError, value_objects::category_summary::CategorySummary, Product},
    infrastructure::pg_product_repo::row::{CategoryRow, ProductRow},
};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn update(pool: &PgPool, product: &Product, category_ids: &[Uuid], updated_by: Uuid) -> Result<Product, ProductRepoError> {
    let mut tx = pool.begin().await.map_err(|e| ProductRepoError::Database(e.to_string()))?;

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
         RETURNING description, id, active, image_url, name, plu, price_cents, sale_mode, symbols, unit_of_measure,
                   '{}'::uuid[] AS category_ids, '{}'::text[] AS category_names",
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
    .fetch_one(&mut *tx)
    .await?;

    sqlx::query("DELETE FROM product_categories WHERE product_id = $1")
        .bind(&product.get_id().value())
        .execute(&mut *tx)
        .await
        .map_err(|e| ProductRepoError::Database(e.to_string()))?;

    sqlx::query(
        "INSERT INTO product_categories (product_id, category_id)
         SELECT $1, unnest($2::uuid[])",
    )
    .bind(&product.get_id().value())
    .bind(category_ids)
    .execute(&mut *tx)
    .await
    .map_err(|e| ProductRepoError::Database(e.to_string()))?;

    let category_rows = sqlx::query_as::<_, CategoryRow>(
        "SELECT id, name FROM categories WHERE id = ANY($1) ORDER BY name",
    )
    .bind(category_ids)
    .fetch_all(&mut *tx)
    .await
    .map_err(|e| ProductRepoError::Database(e.to_string()))?;

    sqlx::query("UPDATE app_settings SET updated_by = $1 WHERE key = 'price_list_updated_at'")
        .bind(updated_by)
        .execute(&mut *tx)
        .await
        .map_err(|e| ProductRepoError::Database(e.to_string()))?;

    tx.commit().await.map_err(|e| ProductRepoError::Database(e.to_string()))?;

    let categories = category_rows.into_iter().map(|r| CategorySummary { id: r.id, name: r.name }).collect();
    Ok(Product::try_from(row)?.with_categories(categories))
}

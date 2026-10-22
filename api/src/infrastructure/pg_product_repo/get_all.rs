use crate::{
    domain::product::{ports::repository::ProductRepoError, Product},
    infrastructure::pg_product_repo::row::ProductRow,
};
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn get_all(
    pool:        &PgPool,
    active_only: bool,
    category_id: Option<Uuid>,
    name:        Option<String>,
    page:        i64,
    per_page:    i64,
) -> Result<(Vec<Product>, i64, Option<DateTime<Utc>>), ProductRepoError> {
    let offset = (page - 1) * per_page;

    let total: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM products p
         WHERE (NOT $1 OR p.active)
           AND ($2::uuid IS NULL OR EXISTS (
                 SELECT 1 FROM product_categories pc2
                 WHERE pc2.product_id = p.id AND pc2.category_id = $2
               ))
           AND ($3::text IS NULL OR p.name ILIKE '%' || $3 || '%')",
    )
    .bind(active_only)
    .bind(category_id)
    .bind(&name)
    .fetch_one(pool)
    .await
    .map_err(|e| ProductRepoError::Database(e.to_string()))?;

    let rows = sqlx::query_as::<_, ProductRow>(
        "SELECT
           p.description, p.id, p.active, p.image_url, p.name, p.plu, p.price_cents, p.sale_mode, p.symbols, p.unit_of_measure,
           COALESCE(array_agg(c.id ORDER BY c.name)   FILTER (WHERE c.id IS NOT NULL), '{}') AS category_ids,
           COALESCE(array_agg(c.name ORDER BY c.name) FILTER (WHERE c.id IS NOT NULL), '{}') AS category_names
         FROM products p
         LEFT JOIN product_categories pc ON pc.product_id = p.id
         LEFT JOIN categories c          ON c.id = pc.category_id
         WHERE (NOT $1 OR p.active)
           AND ($2::uuid IS NULL OR EXISTS (
                 SELECT 1 FROM product_categories pc2
                 WHERE pc2.product_id = p.id AND pc2.category_id = $2
               ))
           AND ($3::text IS NULL OR p.name ILIKE '%' || $3 || '%')
         GROUP BY p.id
         ORDER BY p.name ASC
         LIMIT $5 OFFSET $4",
    )
    .bind(active_only)
    .bind(category_id)
    .bind(&name)
    .bind(offset)
    .bind(per_page)
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

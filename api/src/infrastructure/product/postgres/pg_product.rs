use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::product::{
    aggregate_root::Product, repository::ProductRepo, repository::ProductRepoError,
    value_objects::id::ProductId,
};

use super::row::ProductRow;

pub struct PgProductRepo {
    pool: PgPool,
}

impl PgProductRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProductRepo for PgProductRepo {
    async fn get_all(&self, page: i64, per_page: i64) -> Result<(Vec<Product>, i64), ProductRepoError> {
        let offset     = (page - 1) * per_page;
        let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM products").fetch_one(&self.pool).await.map_err(|_| ProductRepoError::Database)?;
        let rows       = sqlx::query_as::<_, ProductRow>(
            "SELECT description, id, image_url, low_stock_threshold, name, price_cents, stock, symbols, unit_of_measure
             FROM products
             ORDER BY name ASC
             LIMIT $1 OFFSET $2"
        )
        .bind(per_page)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|_| ProductRepoError::Database)?;
        let products = rows.into_iter().map(Product::try_from).collect::<Result<Vec<_>, _>>();
        let products = products.map_err(|e| ProductRepoError::Mapping(e.to_string()))?;
        Ok((products, total))
    }

    async fn get_by_id(&self, product_id: &ProductId) -> Result<Product, ProductRepoError> {
        let row = sqlx::query_as::<_, ProductRow>(
            "SELECT description, id, image_url, low_stock_threshold, name, price_cents, stock, symbols, unit_of_measure
            FROM products
            WHERE id = $1"
        )
        .bind(product_id.value())
        .fetch_one(&self.pool)
        .await
        .map_err(|_| ProductRepoError::Database)?;
        let product = Product::try_from(row)?;
        Ok(product)
    }

    async fn save(&self, product: &Product) -> Result<Product, ProductRepoError> {
        let row = sqlx::query_as::<_, ProductRow>(
            "INSERT INTO products
               (description, id, image_url, low_stock_threshold, name, price_cents, stock, symbols, unit_of_measure)
             VALUES ($1,$2,$3,$4,$5,$6,$7,$8, $9)
             ON CONFLICT (id) DO UPDATE SET
                description         = EXCLUDED.description,
                image_url           = EXCLUDED.image_url,
                low_stock_threshold = EXCLUDED.low_stock_threshold,
                name                = EXCLUDED.name,
                price_cents         = EXCLUDED.price_cents,
                stock               = EXCLUDED.stock,
                symbol              = EXCLUDED.symbol,
                unit_of_measure     = EXCLUDED.unit_of_measure
            RETURNING description, id, image_url, low_stock_threshold, name, price_cents, stock, symbol, unit_of_measure
            ",

        )
        .bind(&product.get_description().map(|d | d.value()))
        .bind(&product.get_id().value())
        .bind(&product.get_image_url())
        .bind(product.get_low_stock_threshold() as i32)
        .bind(&product.get_name().value())
        .bind(product.get_price_cents() as i64)
        .bind(product.get_stock() as i32)
        .bind(&product.get_symbols().iter().map(|symbol| symbol.to_string()).collect::<Vec<String>>())
        .bind(&product.get_unit_of_measure().as_str())
        .fetch_one(&self.pool)
        .await
        .map_err(|_| ProductRepoError::Database)?;
        let product = Product::try_from(row)?;
        Ok(product)
    }
}

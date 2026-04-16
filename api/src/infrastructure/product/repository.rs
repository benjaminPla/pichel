use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::domain::product::{
    repository::ProductRepository, PriceCents, Product, ProductDomainError, ProductId,
    ProductName, UnitAmount, UnitType,
};

pub struct PostgresProductRepository {
    pool: PgPool,
}

impl PostgresProductRepository {
    pub fn new(pool: PgPool) -> Self { Self { pool } }
}

#[async_trait]
impl ProductRepository for PostgresProductRepository {
    async fn save(&self, product: &Product) -> Result<(), ProductDomainError> {
        sqlx::query(
            "INSERT INTO products
               (id, name, description, price_cents, cost_price,
                unit_amount, unit_type, stock, low_stock_threshold, image_url)
             VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)",
        )
        .bind(product.id.0)
        .bind(product.name.value())
        .bind(&product.description)
        .bind(product.price_cents.value())
        .bind(product.cost_price)
        .bind(product.unit_amount.value())
        .bind(product.unit_type.value())
        .bind(product.stock)
        .bind(product.low_stock_threshold)
        .bind(&product.image_url)
        .execute(&self.pool)
        .await
        .map_err(|_| ProductDomainError::Internal)?;
        Ok(())
    }

    async fn find_by_id(&self, id: &ProductId) -> Result<Option<Product>, ProductDomainError> {
        let row = sqlx::query(
            "SELECT id, name, description, price_cents, cost_price,
                    unit_amount, unit_type, stock, low_stock_threshold, image_url
             FROM products WHERE id = $1",
        )
        .bind(id.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| ProductDomainError::Internal)?;

        row.map(|r| map_row(&r)).transpose()
    }

    async fn find_all(&self, page: i64, per_page: i64) -> Result<(Vec<Product>, i64), ProductDomainError> {
        let offset = (page - 1) * per_page;

        let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM products")
            .fetch_one(&self.pool)
            .await
            .map_err(|_| ProductDomainError::Internal)?;

        let rows = sqlx::query(
            "SELECT id, name, description, price_cents, cost_price,
                    unit_amount, unit_type, stock, low_stock_threshold, image_url
             FROM products ORDER BY name
             LIMIT $1 OFFSET $2",
        )
        .bind(per_page)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|_| ProductDomainError::Internal)?;

        let products = rows.iter().map(|r| map_row(r)).collect::<Result<Vec<_>, _>>()?;
        Ok((products, total))
    }
}

fn map_row(r: &sqlx::postgres::PgRow) -> Result<Product, ProductDomainError> {
    Ok(Product {
        id: ProductId(
            r.try_get::<Uuid, _>("id").map_err(|_| ProductDomainError::Internal)?,
        ),
        name: ProductName::new(
            r.try_get::<String, _>("name").map_err(|_| ProductDomainError::Internal)?,
        )?,
        description: r.try_get("description").map_err(|_| ProductDomainError::Internal)?,
        price_cents: PriceCents::new(
            r.try_get::<i32, _>("price_cents").map_err(|_| ProductDomainError::Internal)?,
        )?,
        cost_price: r.try_get("cost_price").map_err(|_| ProductDomainError::Internal)?,
        unit_amount: UnitAmount::new(
            r.try_get::<String, _>("unit_amount").map_err(|_| ProductDomainError::Internal)?,
        )?,
        unit_type: UnitType::new(
            r.try_get::<i32, _>("unit_type").map_err(|_| ProductDomainError::Internal)?,
        ),
        stock: r.try_get::<i32, _>("stock").map_err(|_| ProductDomainError::Internal)?,
        low_stock_threshold: r.try_get::<i32, _>("low_stock_threshold")
            .map_err(|_| ProductDomainError::Internal)?,
        image_url: r.try_get("image_url").map_err(|_| ProductDomainError::Internal)?,
    })
}

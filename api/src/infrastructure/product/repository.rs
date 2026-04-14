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
            "INSERT INTO products (id, name, description, price_cents, unit_amount, unit_type, image_url)
             VALUES ($1, $2, $3, $4, $5, $6, $7)",
        )
        .bind(product.id.0)
        .bind(product.name.value())
        .bind(&product.description)
        .bind(product.price_cents.value())
        .bind(product.unit_amount.value())
        .bind(product.unit_type.value())
        .bind(&product.image_url)
        .execute(&self.pool)
        .await
        .map_err(|_| ProductDomainError::Internal)?;
        Ok(())
    }

    async fn find_by_id(&self, id: &ProductId) -> Result<Option<Product>, ProductDomainError> {
        let row = sqlx::query(
            "SELECT id, name, description, price_cents, unit_amount, unit_type, image_url
             FROM products WHERE id = $1",
        )
        .bind(id.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| ProductDomainError::Internal)?;

        row.map(|r| map_row(&r)).transpose()
    }

    async fn find_all(&self) -> Result<Vec<Product>, ProductDomainError> {
        let rows = sqlx::query(
            "SELECT id, name, description, price_cents, unit_amount, unit_type, image_url
             FROM products ORDER BY name",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|_| ProductDomainError::Internal)?;

        rows.iter().map(|r| map_row(r)).collect()
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
        unit_amount: UnitAmount::new(
            r.try_get::<String, _>("unit_amount").map_err(|_| ProductDomainError::Internal)?,
        )?,
        unit_type: UnitType::new(
            r.try_get::<i32, _>("unit_type").map_err(|_| ProductDomainError::Internal)?,
        ),
        image_url: r.try_get("image_url").map_err(|_| ProductDomainError::Internal)?,
    })
}

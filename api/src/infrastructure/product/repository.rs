use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::domain::product::{
    repository::ProductRepository, Price, Product, ProductDomainError, ProductId, ProductName,
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
        sqlx::query("INSERT INTO products (id, name, price) VALUES ($1, $2, $3)")
            .bind(product.id.0)
            .bind(product.name.value())
            .bind(product.price.value())
            .execute(&self.pool)
            .await
            .map_err(|_| ProductDomainError::Internal)?;
        Ok(())
    }

    async fn find_by_id(&self, id: &ProductId) -> Result<Option<Product>, ProductDomainError> {
        let row = sqlx::query("SELECT id, name, price FROM products WHERE id = $1")
            .bind(id.0)
            .fetch_optional(&self.pool)
            .await
            .map_err(|_| ProductDomainError::Internal)?;

        row.map(|r| {
            Ok(Product {
                id: ProductId(
                    r.try_get::<Uuid, _>("id").map_err(|_| ProductDomainError::Internal)?,
                ),
                name: ProductName::new(
                    r.try_get::<String, _>("name").map_err(|_| ProductDomainError::Internal)?,
                )?,
                price: Price::new(
                    r.try_get::<f64, _>("price").map_err(|_| ProductDomainError::Internal)?,
                )?,
            })
        })
        .transpose()
    }

    async fn find_all(&self) -> Result<Vec<Product>, ProductDomainError> {
        let rows = sqlx::query("SELECT id, name, price FROM products ORDER BY name")
            .fetch_all(&self.pool)
            .await
            .map_err(|_| ProductDomainError::Internal)?;

        rows.into_iter()
            .map(|r| {
                Ok(Product {
                    id: ProductId(
                        r.try_get::<Uuid, _>("id").map_err(|_| ProductDomainError::Internal)?,
                    ),
                    name: ProductName::new(
                        r.try_get::<String, _>("name").map_err(|_| ProductDomainError::Internal)?,
                    )?,
                    price: Price::new(
                        r.try_get::<f64, _>("price").map_err(|_| ProductDomainError::Internal)?,
                    )?,
                })
            })
            .collect()
    }
}

mod create;
mod delete;
mod errors;
mod get_all;
mod get_by_id;
mod row;
mod update;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use crate::domain::product::{
    ports::repository::{ProductRepo, ProductRepoError},
    value_objects::id::ProductId,
    Product,
};

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
    async fn create(&self, product: &Product) -> Result<Product, ProductRepoError> {
        create::create(&self.pool, product).await
    }

    async fn delete(&self, product_id: &ProductId) -> Result<Product, ProductRepoError> {
        delete::delete(&self.pool, product_id).await
    }

    async fn get_all(&self, page: i64, per_page: i64) -> Result<(Vec<Product>, i64, Option<DateTime<Utc>>), ProductRepoError> {
        get_all::get_all(&self.pool, page, per_page).await
    }

    async fn get_by_id(&self, product_id: &ProductId) -> Result<Product, ProductRepoError> {
        get_by_id::get_by_id(&self.pool, product_id).await
    }

    async fn update(&self, product: &Product) -> Result<Product, ProductRepoError> {
        update::update(&self.pool, product).await
    }
}

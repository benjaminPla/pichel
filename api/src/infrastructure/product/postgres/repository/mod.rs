mod create;
mod get_all;
mod get_by_id;
mod update;

use async_trait::async_trait;
use sqlx::PgPool;
use create::create;
use get_all::get_all;
use get_by_id::get_by_id;
use update::update;
use crate::domain::product::{
    aggregate_root::Product,
    ports::repository::{ProductRepo, ProductRepoError},
    value_objects::id::ProductId,
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
        create(&self.pool, product).await
    }

    async fn get_all(&self, page: i64, per_page: i64) -> Result<(Vec<Product>, i64), ProductRepoError> {
        get_all(&self.pool, page, per_page).await
    }

    async fn get_by_id(&self, product_id: &ProductId) -> Result<Product, ProductRepoError> {
        get_by_id(&self.pool, product_id).await
    }

    async fn update(&self, product: &Product) -> Result<Product, ProductRepoError> {
        update(&self.pool, product).await
    }
}

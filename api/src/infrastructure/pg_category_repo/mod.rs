mod create;
mod delete;
mod errors;
mod get_all;
mod row;
mod update;

use async_trait::async_trait;
use sqlx::PgPool;
use crate::domain::category::{
    ports::repository::{CategoryRepo, CategoryRepoError},
    value_objects::id::CategoryId,
    Category,
};

pub struct PgCategoryRepo {
    pool: PgPool,
}

impl PgCategoryRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CategoryRepo for PgCategoryRepo {
    async fn create(&self, category: &Category) -> Result<Category, CategoryRepoError> {
        create::create(&self.pool, category).await
    }

    async fn delete(&self, category_id: &CategoryId) -> Result<(), CategoryRepoError> {
        delete::delete(&self.pool, category_id).await
    }

    async fn get_all(&self) -> Result<Vec<Category>, CategoryRepoError> {
        get_all::get_all(&self.pool).await
    }

    async fn update(&self, category: &Category) -> Result<Category, CategoryRepoError> {
        update::update(&self.pool, category).await
    }
}

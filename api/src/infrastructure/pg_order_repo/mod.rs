mod create;
mod errors;
mod get_all;
mod get_by_id;
mod row;

use async_trait::async_trait;
use sqlx::PgPool;
use crate::domain::order::{
    ports::repository::{OrderRepo, OrderRepoError},
    value_objects::id::OrderId,
    Order, OrderItem,
};

pub struct PgOrderRepo {
    pool: PgPool,
}

impl PgOrderRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl OrderRepo for PgOrderRepo {
    async fn create(&self, order: &Order, items: &[OrderItem]) -> Result<(Order, Vec<OrderItem>), OrderRepoError> {
        create::create(&self.pool, order, items).await
    }

    async fn get_all(&self, page: i64, per_page: i64) -> Result<(Vec<(Order, Vec<OrderItem>)>, i64), OrderRepoError> {
        get_all::get_all(&self.pool, page, per_page).await
    }

    async fn get_by_id(&self, order_id: &OrderId) -> Result<(Order, Vec<OrderItem>), OrderRepoError> {
        get_by_id::get_by_id(&self.pool, order_id).await
    }
}

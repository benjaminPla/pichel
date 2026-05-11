mod create;
mod errors;
mod get_by_id;
mod list;
mod row;
mod update_status;

use async_trait::async_trait;
use sqlx::PgPool;
use crate::domain::order::{
    ports::repository::{OrderRepo, OrderRepoError},
    value_objects::{id::OrderId, order_status::OrderStatus},
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

    async fn get_by_id(&self, order_id: &OrderId) -> Result<(Order, Vec<OrderItem>), OrderRepoError> {
        get_by_id::get_by_id(&self.pool, order_id).await
    }

    async fn list(&self, page: i64, per_page: i64) -> Result<(Vec<(Order, Vec<OrderItem>)>, i64), OrderRepoError> {
        list::list(&self.pool, page, per_page).await
    }

    async fn update_status(&self, order_id: &OrderId, status: &OrderStatus) -> Result<Order, OrderRepoError> {
        update_status::update_status(&self.pool, order_id, status).await
    }
}

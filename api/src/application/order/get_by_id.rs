use std::sync::Arc;
use uuid::Uuid;
use crate::{
    application::order::errors::OrderAppError,
    domain::order::{
        ports::repository::OrderRepo,
        value_objects::id::OrderId,
        Order, OrderItem,
    },
};

pub struct GetOrderByIdInput {
    pub id: Uuid,
}

pub struct GetOrderByIdUseCase {
    order_repo: Arc<dyn OrderRepo>,
}

impl GetOrderByIdUseCase {
    pub fn new(order_repo: Arc<dyn OrderRepo>) -> Self {
        Self { order_repo }
    }

    pub async fn execute(&self, input: GetOrderByIdInput) -> Result<(Order, Vec<OrderItem>), OrderAppError> {
        let order_id = OrderId::reconstitute(input.id);
        let result   = self.order_repo.get_by_id(&order_id).await?;
        Ok(result)
    }
}

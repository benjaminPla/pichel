use std::sync::Arc;
use uuid::Uuid;
use crate::{
    application::order::errors::OrderAppError,
    domain::order::{
        ports::repository::OrderRepo,
        value_objects::{id::OrderId, order_status::OrderStatus},
        Order,
    },
};

pub struct UpdateOrderStatusInput {
    pub id:     Uuid,
    pub status: String,
}

pub struct UpdateOrderStatusUseCase {
    order_repo: Arc<dyn OrderRepo>,
}

impl UpdateOrderStatusUseCase {
    pub fn new(order_repo: Arc<dyn OrderRepo>) -> Self {
        Self { order_repo }
    }

    pub async fn execute(&self, input: UpdateOrderStatusInput) -> Result<Order, OrderAppError> {
        let order_id   = OrderId::reconstitute(input.id);
        let new_status = input.status.parse::<OrderStatus>()?;

        let (current, _) = self.order_repo.get_by_id(&order_id).await?;

        validate_transition(current.get_status(), &new_status)?;

        let order = self.order_repo.update_status(&order_id, &new_status).await?;
        Ok(order)
    }
}

fn validate_transition(current: &OrderStatus, next: &OrderStatus) -> Result<(), OrderAppError> {
    let allowed = match current {
        OrderStatus::Pending   => matches!(next, OrderStatus::Closed | OrderStatus::Cancelled),
        OrderStatus::Closed    => matches!(next, OrderStatus::Cancelled),
        OrderStatus::Cancelled => false,
    };
    if allowed { Ok(()) } else { Err(OrderAppError::InvalidStatusTransition) }
}

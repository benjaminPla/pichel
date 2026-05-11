use std::sync::Arc;
use crate::{
    application::order::errors::OrderAppError,
    domain::order::{ports::repository::OrderRepo, Order, OrderItem},
};

pub struct ListOrdersInput {
    pub page:     i64,
    pub per_page: i64,
}

pub struct ListOrdersUseCase {
    order_repo: Arc<dyn OrderRepo>,
}

impl ListOrdersUseCase {
    pub fn new(order_repo: Arc<dyn OrderRepo>) -> Self {
        Self { order_repo }
    }

    pub async fn execute(&self, input: ListOrdersInput) -> Result<(Vec<(Order, Vec<OrderItem>)>, i64), OrderAppError> {
        let result = self.order_repo.list(input.page, input.per_page).await?;
        Ok(result)
    }
}

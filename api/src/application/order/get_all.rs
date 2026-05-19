use std::sync::Arc;
use crate::{
    application::order::errors::OrderAppError,
    domain::order::{ports::repository::OrderRepo, Order, OrderItem},
};

pub struct GetAllOrdersInput {
    pub page:     i64,
    pub per_page: i64,
}

pub struct GetAllOrdersUseCase {
    order_repo: Arc<dyn OrderRepo>,
}

impl GetAllOrdersUseCase {
    pub fn new(order_repo: Arc<dyn OrderRepo>) -> Self {
        Self { order_repo }
    }

    pub async fn execute(&self, input: GetAllOrdersInput) -> Result<(Vec<(Order, Vec<OrderItem>)>, i64), OrderAppError> {
        let result = self.order_repo.get_all(input.page, input.per_page).await?;
        Ok(result)
    }
}

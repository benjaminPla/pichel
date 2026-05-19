use crate::domain::order::{
    value_objects::{
        id::OrderId,
        order_status::OrderStatusError,
        quantity::QuantityError,
    },
    Order, OrderItem,
};

use crate::domain::product::value_objects::price_cents::PriceCentsError;
use async_trait::async_trait;

#[async_trait]
pub trait OrderRepo: Send + Sync {
    async fn create(&self, order: &Order, items: &[OrderItem]) -> Result<(Order, Vec<OrderItem>), OrderRepoError>;
    async fn get_all(&self, page: i64, per_page: i64)          -> Result<(Vec<(Order, Vec<OrderItem>)>, i64), OrderRepoError>;
    async fn get_by_id(&self, order_id: &OrderId)              -> Result<(Order, Vec<OrderItem>), OrderRepoError>;
}

#[derive(Debug, thiserror::Error)]
pub enum OrderRepoError {
    #[error("internal server error")]
    Database(String),
    #[error("not found")]
    NotFound,
    #[error("{0}")]
    Mapping(String),
}

impl From<OrderStatusError> for OrderRepoError {
    fn from(e: OrderStatusError) -> Self { Self::Mapping(e.to_string()) }
}

impl From<PriceCentsError> for OrderRepoError {
    fn from(e: PriceCentsError) -> Self { Self::Mapping(e.to_string()) }
}

impl From<QuantityError> for OrderRepoError {
    fn from(e: QuantityError) -> Self { Self::Mapping(e.to_string()) }
}

impl From<crate::domain::product::value_objects::sale_mode::SaleModeError> for OrderRepoError {
    fn from(e: crate::domain::product::value_objects::sale_mode::SaleModeError) -> Self { Self::Mapping(e.to_string()) }
}

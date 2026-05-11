use crate::domain::order::{
    ports::repository::OrderRepoError,
    value_objects::{order_status::OrderStatusError, quantity::QuantityError},
};
use crate::domain::product::value_objects::price_cents::PriceCentsError;

#[derive(Debug, thiserror::Error)]
pub enum OrderAppError {
    #[error("internal server error")]
    Internal(String),
    #[error("not found")]
    NotFound,
    #[error("{0}")]
    Validation(String),
    #[error("invalid status transition")]
    InvalidStatusTransition,
}

impl From<OrderRepoError> for OrderAppError {
    fn from(e: OrderRepoError) -> Self {
        match e {
            OrderRepoError::Database(msg) => Self::Internal(msg),
            OrderRepoError::Mapping(msg)  => Self::Internal(msg),
            OrderRepoError::NotFound      => Self::NotFound,
        }
    }
}

impl From<OrderStatusError> for OrderAppError {
    fn from(e: OrderStatusError) -> Self { Self::Validation(e.to_string()) }
}

impl From<PriceCentsError> for OrderAppError {
    fn from(e: PriceCentsError) -> Self { Self::Validation(e.to_string()) }
}

impl From<QuantityError> for OrderAppError {
    fn from(e: QuantityError) -> Self { Self::Validation(e.to_string()) }
}

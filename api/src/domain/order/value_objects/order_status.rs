use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum OrderStatus {
    Pending,
    Closed,
    Cancelled,
}

impl OrderStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending   => "pending",
            Self::Closed    => "closed",
            Self::Cancelled => "cancelled",
        }
    }
}

impl FromStr for OrderStatus {
    type Err = OrderStatusError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pending"   => Ok(Self::Pending),
            "closed"    => Ok(Self::Closed),
            "cancelled" => Ok(Self::Cancelled),
            other       => Err(OrderStatusError::Invalid(other.to_string())),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum OrderStatusError {
    #[error("invalid order status: {0}")]
    Invalid(String),
}

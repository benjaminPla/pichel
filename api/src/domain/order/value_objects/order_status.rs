use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum OrderStatus {
    Cancelled,
    Closed,
    Pending,
}

impl OrderStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Cancelled => "cancelled",
            Self::Closed    => "closed",
            Self::Pending   => "pending",
        }
    }
}

impl FromStr for OrderStatus {
    type Err = OrderStatusError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "cancelled" => Ok(Self::Cancelled),
            "closed"    => Ok(Self::Closed),
            "pending"   => Ok(Self::Pending),
            other       => Err(OrderStatusError::Invalid(other.to_string())),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum OrderStatusError {
    #[error("invalid order status: {0}")]
    Invalid(String),
}

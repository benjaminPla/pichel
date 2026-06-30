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

#[derive(Debug, thiserror::Error)]
pub enum OrderStatusTransitionError {
    #[error("cannot transition order from '{from}' to '{to}'")]
    Invalid { from: String, to: String },
}

impl OrderStatus {
    pub fn validate_transition(&self, next: &OrderStatus) -> Result<(), OrderStatusTransitionError> {
        match (self, next) {
            (Self::Pending, Self::Closed)    => Ok(()),
            (Self::Pending, Self::Cancelled) => Ok(()),
            (a, b) if a == b               => Ok(()),
            _ => Err(OrderStatusTransitionError::Invalid {
                from: self.as_str().to_string(),
                to:   next.as_str().to_string(),
            }),
        }
    }
}

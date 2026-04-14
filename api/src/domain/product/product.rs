use thiserror::Error;
use uuid::Uuid;

// ── Value Objects ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProductId(pub Uuid);

#[derive(Debug, Clone)]
pub struct ProductName(String);

impl ProductName {
    pub fn new(value: impl Into<String>) -> Result<Self, ProductDomainError> {
        let s = value.into();
        if s.trim().is_empty() {
            Err(ProductDomainError::EmptyName)
        } else {
            Ok(Self(s))
        }
    }
    pub fn value(&self) -> &str { &self.0 }
}

/// Use `rust_decimal::Decimal` in production for monetary amounts.
#[derive(Debug, Clone)]
pub struct Price(f64);

impl Price {
    pub fn new(value: f64) -> Result<Self, ProductDomainError> {
        if value < 0.0 {
            Err(ProductDomainError::NegativePrice)
        } else {
            Ok(Self(value))
        }
    }
    pub fn value(&self) -> f64 { self.0 }
}

// ── Aggregate Root ────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Product {
    pub id: ProductId,
    pub name: ProductName,
    pub price: Price,
}

impl Product {
    pub fn create(name: ProductName, price: Price) -> Self {
        Self { id: ProductId(Uuid::new_v4()), name, price }
    }
}

// ── Errors ────────────────────────────────────────────────────────────────────

#[derive(Debug, Error)]
pub enum ProductDomainError {
    #[error("product name cannot be empty")]
    EmptyName,
    #[error("price must be non-negative")]
    NegativePrice,
    #[error("product not found")]
    NotFound,
    #[error("internal error")]
    Internal,
}

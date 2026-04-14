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

#[derive(Debug, Clone)]
pub struct PriceCents(i32);

impl PriceCents {
    pub fn new(value: i32) -> Result<Self, ProductDomainError> {
        if value < 0 {
            Err(ProductDomainError::NegativePrice)
        } else {
            Ok(Self(value))
        }
    }
    pub fn value(&self) -> i32 { self.0 }
}

#[derive(Debug, Clone)]
pub struct UnitAmount(String);

impl UnitAmount {
    pub fn new(value: impl Into<String>) -> Result<Self, ProductDomainError> {
        let s = value.into();
        if s.trim().is_empty() {
            Err(ProductDomainError::EmptyUnitAmount)
        } else {
            Ok(Self(s))
        }
    }
    pub fn value(&self) -> &str { &self.0 }
}

#[derive(Debug, Clone)]
pub struct UnitType(i32);

impl UnitType {
    pub fn new(value: i32) -> Self { Self(value) }
    pub fn value(&self) -> i32 { self.0 }
}

// ── Aggregate Root ────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Product {
    pub id:          ProductId,
    pub name:        ProductName,
    pub description: Option<String>,
    pub price_cents: PriceCents,
    pub unit_amount: UnitAmount,
    pub unit_type:   UnitType,
    pub image_url:   Option<String>,
}

impl Product {
    pub fn create(
        name:        ProductName,
        description: Option<String>,
        price_cents: PriceCents,
        unit_amount: UnitAmount,
        unit_type:   UnitType,
        image_url:   Option<String>,
    ) -> Self {
        Self {
            id: ProductId(Uuid::new_v4()),
            name,
            description,
            price_cents,
            unit_amount,
            unit_type,
            image_url,
        }
    }
}

// ── Errors ────────────────────────────────────────────────────────────────────

#[derive(Debug, Error)]
pub enum ProductDomainError {
    #[error("product name cannot be empty")]
    EmptyName,
    #[error("price must be non-negative")]
    NegativePrice,
    #[error("unit amount cannot be empty")]
    EmptyUnitAmount,
    #[error("product not found")]
    NotFound,
    #[error("internal error")]
    Internal,
}

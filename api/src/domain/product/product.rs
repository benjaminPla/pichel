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

// ── Stock status ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum StockStatus {
    InStock,
    LowStock,
    OutOfStock,
}

impl StockStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::InStock    => "in_stock",
            Self::LowStock   => "low_stock",
            Self::OutOfStock => "out_of_stock",
        }
    }
}

// ── Aggregate Root ────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Product {
    pub id:                  ProductId,
    pub name:                ProductName,
    pub description:         Option<String>,
    pub price_cents:         PriceCents,
    pub cost_price:          Option<i32>,   // what the store paid; null = unknown
    pub unit_amount:         UnitAmount,
    pub unit_type:           UnitType,
    pub stock:               i32,           // current on-hand quantity
    pub low_stock_threshold: i32,           // warn when stock drops to/below this
    pub image_url:           Option<String>,
}

impl Product {
    pub fn create(
        name:                ProductName,
        description:         Option<String>,
        price_cents:         PriceCents,
        cost_price:          Option<i32>,
        unit_amount:         UnitAmount,
        unit_type:           UnitType,
        stock:               i32,
        low_stock_threshold: i32,
        image_url:           Option<String>,
    ) -> Self {
        Self {
            id: ProductId(Uuid::new_v4()),
            name,
            description,
            price_cents,
            cost_price,
            unit_amount,
            unit_type,
            stock,
            low_stock_threshold,
            image_url,
        }
    }

    pub fn stock_status(&self) -> StockStatus {
        if self.stock == 0 {
            StockStatus::OutOfStock
        } else if self.stock <= self.low_stock_threshold {
            StockStatus::LowStock
        } else {
            StockStatus::InStock
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

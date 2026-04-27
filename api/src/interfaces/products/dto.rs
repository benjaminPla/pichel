use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::product::aggregate_root::Product;

// ── ProductCreate ────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct ProductCreateRequestBody {
    pub description:         Option<String>,
    pub image_url:           Option<String>,
    pub low_stock_threshold: u32,
    pub name:                String,
    pub price_cents:         u64,
    pub stock:               u32,
    pub symbols:             Vec<String>,
    pub unit_of_measure:     String,
}

#[derive(Serialize)]
pub struct ProductCreateResponse {
    description:         Option<String>,
    id:                  Uuid,
    image_url:           Option<String>,
    low_stock_threshold: u32,
    name:                String,
    price_cents:         u64,
    stock:               u32,
    symbols:             Vec<String>,
    unit_of_measure:     String,
}

impl From<Product> for ProductCreateResponse {
    fn from(p: Product) -> Self {
        Self {
            description:         p.get_description().map(|d| d.value().to_string()),
            id:                  p.get_id().value(),
            image_url:           p.get_image_url().map(|i| i.to_string()),
            low_stock_threshold: p.get_low_stock_threshold(),
            name:                p.get_name().value().to_string(),
            price_cents:         p.get_price_cents(),
            stock:               p.get_stock(),
            symbols:             p.get_symbols().into_iter().map(|s| s.to_string()).collect(),
            unit_of_measure:     p.get_unit_of_measure().as_str().to_string(),
        }
    }
}

// ── ProductGetAll ────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct ProductGetAllQueryParams {
    pub page:     i64,
    pub per_page: i64,
}

#[derive(Serialize)]
pub struct ProductGetAllItem {
    description:         Option<String>,
    id:                  Uuid,
    image_url:           Option<String>,
    low_stock_threshold: u32,
    name:                String,
    price_cents:         u64,
    stock:               u32,
    symbols:             Vec<String>,
    unit_of_measure:     String,
}

impl From<Product> for ProductGetAllItem {
    fn from(p: Product) -> Self {
        Self {
            description:         p.get_description().map(|d| d.value().to_string()),
            id:                  p.get_id().value(),
            image_url:           p.get_image_url().map(|i| i.to_string()),
            low_stock_threshold: p.get_low_stock_threshold(),
            name:                p.get_name().value().to_string(),
            price_cents:         p.get_price_cents(),
            stock:               p.get_stock(),
            symbols:             p.get_symbols().into_iter().map(|s| s.to_string()).collect(),
            unit_of_measure:     p.get_unit_of_measure().as_str().to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct ProductGetAllResponse {
    pub products: Vec<ProductGetAllItem>,
    pub total:    i64,
}

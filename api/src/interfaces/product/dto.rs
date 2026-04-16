use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ── Pagination ────────────────────────────────────────────────────────────────

#[derive(Serialize)]
pub struct Paginated<T: Serialize> {
    pub data:        Vec<T>,
    pub total:       i64,
    pub page:        i64,
    pub per_page:    i64,
    pub total_pages: i64,
}

use crate::domain::product::Product;

// ── Create ────────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct CreateProductRequest {
    pub name:                String,
    pub description:         Option<String>,
    pub price_cents:         i32,
    pub cost_price:          Option<i32>,
    pub unit_amount:         String,
    pub unit_type:           i32,
    #[serde(default)]
    pub stock:               i32,
    #[serde(default = "default_low_stock_threshold")]
    pub low_stock_threshold: i32,
    pub image_url:           Option<String>,
}

fn default_low_stock_threshold() -> i32 { 5 }

// ── Public response (no cost_price, computed stock_status) ────────────────────

#[derive(Serialize)]
pub struct PublicProductResponse {
    pub id:           Uuid,
    pub name:         String,
    pub description:  Option<String>,
    pub price_cents:  i32,
    pub unit_amount:  String,
    pub unit_type:    i32,
    pub image_url:    Option<String>,
    pub stock_status: String,   // "in_stock" | "low_stock" | "out_of_stock"
}

impl From<Product> for PublicProductResponse {
    fn from(p: Product) -> Self {
        let status = p.stock_status().as_str().to_string();
        Self {
            id:           p.id.0,
            name:         p.name.value().to_string(),
            description:  p.description,
            price_cents:  p.price_cents.value(),
            unit_amount:  p.unit_amount.value().to_string(),
            unit_type:    p.unit_type.value(),
            image_url:    p.image_url,
            stock_status: status,
        }
    }
}

// ── Admin response (all fields) ───────────────────────────────────────────────

#[derive(Serialize)]
pub struct AdminProductResponse {
    pub id:                  Uuid,
    pub name:                String,
    pub description:         Option<String>,
    pub price_cents:         i32,
    pub cost_price:          Option<i32>,
    pub unit_amount:         String,
    pub unit_type:           i32,
    pub stock:               i32,
    pub low_stock_threshold: i32,
    pub image_url:           Option<String>,
    pub stock_status:        String,
}

impl From<Product> for AdminProductResponse {
    fn from(p: Product) -> Self {
        let status = p.stock_status().as_str().to_string();
        Self {
            id:                  p.id.0,
            name:                p.name.value().to_string(),
            description:         p.description,
            price_cents:         p.price_cents.value(),
            cost_price:          p.cost_price,
            unit_amount:         p.unit_amount.value().to_string(),
            unit_type:           p.unit_type.value(),
            stock:               p.stock,
            low_stock_threshold: p.low_stock_threshold,
            image_url:           p.image_url,
            stock_status:        status,
        }
    }
}

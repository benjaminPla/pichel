use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::product::Product;

#[derive(Deserialize)]
pub struct CreateProductRequest {
    pub name:        String,
    pub description: Option<String>,
    pub price_cents: i32,
    pub unit_amount: String,
    pub unit_type:   i32,
    pub image_url:   Option<String>,
}

#[derive(Serialize)]
pub struct ProductResponse {
    pub id:          Uuid,
    pub name:        String,
    pub description: Option<String>,
    pub price_cents: i32,
    pub unit_amount: String,
    pub unit_type:   i32,
    pub image_url:   Option<String>,
}

impl From<Product> for ProductResponse {
    fn from(p: Product) -> Self {
        Self {
            id:          p.id.0,
            name:        p.name.value().to_string(),
            description: p.description,
            price_cents: p.price_cents.value(),
            unit_amount: p.unit_amount.value().to_string(),
            unit_type:   p.unit_type.value(),
            image_url:   p.image_url,
        }
    }
}

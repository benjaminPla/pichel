use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::product::Product;

#[derive(Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub price: f64,
}

#[derive(Serialize)]
pub struct ProductResponse {
    pub id: Uuid,
    pub name: String,
    pub price: f64,
}

impl From<Product> for ProductResponse {
    fn from(p: Product) -> Self {
        Self { id: p.id.0, name: p.name.value().to_string(), price: p.price.value() }
    }
}

use serde::Serialize;
use uuid::Uuid;
use crate::domain::product::Product;

#[derive(Serialize)]
pub struct ProductGetByIdResponse {
    description:         Option<String>,
    id:                  Uuid,
    image_url:           Option<String>,
    low_stock_threshold: u32,
    name:                String,
    price_cents:         u32,
    stock:               u32,
    stock_status:        String,
    symbols:             Vec<String>,
    unit_of_measure:     String,
}

impl From<Product> for ProductGetByIdResponse {
    fn from(p: Product) -> Self {
        Self {
            description:         p.get_description().map(|d| d.value().to_string()),
            id:                  p.get_id().value(),
            image_url:           p.get_image_url().map(|i| i.to_string()),
            low_stock_threshold: p.get_low_stock_threshold(),
            name:                p.get_name().value().to_string(),
            price_cents:         p.get_price_cents(),
            stock:               p.get_stock(),
            stock_status:        p.compute_stock_status().as_str().to_string(),
            symbols:             p.get_symbols().into_iter().map(|s| s.to_string()).collect(),
            unit_of_measure:     p.get_unit_of_measure().as_str().to_string(),
        }
    }
}

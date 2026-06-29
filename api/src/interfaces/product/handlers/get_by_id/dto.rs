use serde::Serialize;
use uuid::Uuid;
use crate::domain::product::Product;

#[derive(Serialize)]
pub struct ProductGetByIdResponse {
    id:              Uuid,
    image_url:       Option<String>,
    name:            String,
    price_cents:     u32,
    sale_mode:       String,
    symbols:         Vec<String>,
    unit_of_measure: String,
}

impl From<Product> for ProductGetByIdResponse {
    fn from(p: Product) -> Self {
        Self {
            id:              p.get_id().value(),
            image_url:       p.get_image_url().map(|i| i.to_string()),
            name:            p.get_name().value().to_string(),
            price_cents:     p.get_price_cents().value(),
            sale_mode:       p.get_sale_mode().as_str().to_string(),
            symbols:         p.get_symbols().iter().map(|s| s.to_string()).collect(),
            unit_of_measure: p.get_unit_of_measure().as_str().to_string(),
        }
    }
}

use crate::domain::product::Product;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct ProductGetAllItem {
    description:     Option<String>,
    id:              Uuid,
    image_url:       Option<String>,
    name:            String,
    price_cents:     u32,
    sale_mode:       String,
    symbols:         Vec<String>,
    unit_of_measure: String,
}

impl From<Product> for ProductGetAllItem {
    fn from(p: Product) -> Self {
        Self {
            description:     p.get_description().map(|d| d.value().to_string()),
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

#[derive(Serialize)]
pub struct ProductGetAllResponse {
    pub products:              Vec<ProductGetAllItem>,
    pub total:                 i64,
    pub price_list_updated_at: Option<String>,
}

#[derive(Deserialize)]
pub struct GetAllQueryParams {
    pub page:     i64,
    pub per_page: i64,
}

impl GetAllQueryParams {
    pub const MAX_PER_PAGE: i64 = 50;
}

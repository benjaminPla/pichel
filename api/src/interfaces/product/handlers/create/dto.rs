use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::product::Product;

#[derive(Deserialize)]
pub struct ProductCreateRequestBody {
    pub active:      Option<bool>,
    #[serde(default)]
    pub category_ids: Vec<Uuid>,
    pub description: Option<String>,
    pub image_url:   Option<String>,
    pub name:        String,
    pub price_cents: u32,
    pub sale_mode:   String,
    pub symbols:     Vec<String>,
}

#[derive(Serialize)]
pub struct CategoryDto {
    pub id:   Uuid,
    pub name: String,
}

#[derive(Serialize)]
pub struct ProductCreateResponse {
    categories:      Vec<CategoryDto>,
    description:     Option<String>,
    id:              Uuid,
    active:          bool,
    image_url:       Option<String>,
    name:            String,
    plu:             u16,
    price_cents:     u32,
    sale_mode:       String,
    symbols:         Vec<String>,
    unit_of_measure: String,
}

impl From<Product> for ProductCreateResponse {
    fn from(p: Product) -> Self {
        Self {
            categories:      p.get_categories().iter().map(|c| CategoryDto { id: c.id, name: c.name.clone() }).collect(),
            description:     p.get_description().map(|d| d.value().to_string()),
            id:              p.get_id().value(),
            active:          p.get_active(),
            image_url:       p.get_image_url().map(|i| i.to_string()),
            name:            p.get_name().value().to_string(),
            plu:             p.get_plu().expect("persisted product always has plu").value(),
            price_cents:     p.get_price_cents().value(),
            sale_mode:       p.get_sale_mode().as_str().to_string(),
            symbols:         p.get_symbols().iter().map(|s| s.to_string()).collect(),
            unit_of_measure: p.get_unit_of_measure().as_str().to_string(),
        }
    }
}

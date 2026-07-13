use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::category::Category;

#[derive(Deserialize)]
pub struct CategoryCreateRequestBody {
    pub name: String,
}

#[derive(Serialize)]
pub struct CategoryCreateResponse {
    id:   Uuid,
    name: String,
}

impl From<Category> for CategoryCreateResponse {
    fn from(c: Category) -> Self {
        Self {
            id:   c.get_id().value(),
            name: c.get_name().value().to_string(),
        }
    }
}

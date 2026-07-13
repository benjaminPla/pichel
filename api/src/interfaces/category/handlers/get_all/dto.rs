use serde::Serialize;
use uuid::Uuid;
use crate::domain::category::Category;

#[derive(Serialize)]
pub struct CategoryGetAllItem {
    id:   Uuid,
    name: String,
}

impl From<Category> for CategoryGetAllItem {
    fn from(c: Category) -> Self {
        Self {
            id:   c.get_id().value(),
            name: c.get_name().value().to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct CategoryGetAllResponse {
    pub categories: Vec<CategoryGetAllItem>,
}

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CategorySummary {
    pub id:   Uuid,
    pub name: String,
}

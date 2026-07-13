use uuid::Uuid;
use crate::domain::category::{
    ports::repository::CategoryRepoError,
    value_objects::{id::CategoryId, name::Name},
    Category,
};

#[derive(sqlx::FromRow)]
pub struct CategoryRow {
    pub id:   Uuid,
    pub name: String,
}

impl TryFrom<CategoryRow> for Category {
    type Error = CategoryRepoError;

    fn try_from(r: CategoryRow) -> Result<Category, CategoryRepoError> {
        let id   = CategoryId::reconstitute(r.id);
        let name = Name::new(r.name)?;
        Ok(Category::reconstitute(id, name))
    }
}

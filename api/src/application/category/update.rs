use crate::{
    application::category::errors::CategoryAppError,
    domain::category::{
        ports::repository::CategoryRepo,
        value_objects::{id::CategoryId, name::Name},
        Category,
    },
};
use std::sync::Arc;
use uuid::Uuid;

pub struct UpdateCategoryInput {
    pub id:   Uuid,
    pub name: String,
}

pub struct UpdateCategoryUseCase {
    category_repo: Arc<dyn CategoryRepo>,
}

impl UpdateCategoryUseCase {
    pub fn new(category_repo: Arc<dyn CategoryRepo>) -> Self {
        Self { category_repo }
    }

    pub async fn execute(&self, input: UpdateCategoryInput) -> Result<Category, CategoryAppError> {
        let id       = CategoryId::reconstitute(input.id);
        let name     = Name::new(input.name)?;
        let category = Category::reconstitute(id, name);
        let category = self.category_repo.update(&category).await?;
        Ok(category)
    }
}

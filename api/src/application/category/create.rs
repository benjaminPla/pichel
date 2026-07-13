use crate::{
    application::category::errors::CategoryAppError,
    domain::category::{ports::repository::CategoryRepo, value_objects::name::Name, Category},
};
use std::sync::Arc;

pub struct CreateCategoryInput {
    pub name: String,
}

pub struct CreateCategoryUseCase {
    category_repo: Arc<dyn CategoryRepo>,
}

impl CreateCategoryUseCase {
    pub fn new(category_repo: Arc<dyn CategoryRepo>) -> Self {
        Self { category_repo }
    }

    pub async fn execute(&self, input: CreateCategoryInput) -> Result<Category, CategoryAppError> {
        let name     = Name::new(input.name)?;
        let category = Category::new(name);
        let category = self.category_repo.create(&category).await?;
        Ok(category)
    }
}

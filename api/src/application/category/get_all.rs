use crate::{
    application::category::errors::CategoryAppError,
    domain::category::{ports::repository::CategoryRepo, Category},
};
use std::sync::Arc;

pub struct GetAllCategoriesUseCase {
    category_repo: Arc<dyn CategoryRepo>,
}

impl GetAllCategoriesUseCase {
    pub fn new(category_repo: Arc<dyn CategoryRepo>) -> Self {
        Self { category_repo }
    }

    pub async fn execute(&self) -> Result<Vec<Category>, CategoryAppError> {
        let categories = self.category_repo.get_all().await?;
        Ok(categories)
    }
}

use crate::{
    application::category::errors::CategoryAppError,
    domain::category::{ports::repository::CategoryRepo, value_objects::id::CategoryId},
};
use std::sync::Arc;
use uuid::Uuid;

pub struct DeleteCategoryInput {
    pub id: Uuid,
}

pub struct DeleteCategoryUseCase {
    category_repo: Arc<dyn CategoryRepo>,
}

impl DeleteCategoryUseCase {
    pub fn new(category_repo: Arc<dyn CategoryRepo>) -> Self {
        Self { category_repo }
    }

    pub async fn execute(&self, input: DeleteCategoryInput) -> Result<(), CategoryAppError> {
        let id = CategoryId::reconstitute(input.id);
        self.category_repo.delete(&id).await?;
        Ok(())
    }
}

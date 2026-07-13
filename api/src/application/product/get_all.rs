use crate::{
    application::product::errors::ProductAppError,
    domain::product::{ports::repository::ProductRepo, Product},
};
use chrono::{DateTime, Utc};
use std::sync::Arc;
use uuid::Uuid;

pub struct GetAllProductsInput {
    pub page:        i64,
    pub per_page:    i64,
    pub name:        Option<String>,
    pub category_id: Option<Uuid>,
}

pub struct GetAllProductsUseCase {
    product_repo: Arc<dyn ProductRepo>,
}

impl GetAllProductsUseCase {
    pub fn new(product_repo: Arc<dyn ProductRepo>) -> Self {
        Self { product_repo }
    }

    pub async fn execute(&self, input: GetAllProductsInput) -> Result<(Vec<Product>, i64, Option<DateTime<Utc>>), ProductAppError> {
        let name = input.name.filter(|s| !s.trim().is_empty());
        let (products, total, price_list_updated_at) = self.product_repo
            .get_all(input.page, input.per_page, name, input.category_id)
            .await?;
        Ok((products, total, price_list_updated_at))
    }
}

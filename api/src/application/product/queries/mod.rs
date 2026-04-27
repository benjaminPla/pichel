use std::sync::Arc;

use crate::{
    application::product::errors::ProductAppError,
    domain::product::{aggregate_root::Product, repository::ProductRepo},
};

// ── Query ────────────────────────────────────────────────────────────────

pub struct ProductGetAllQuery {
    page: i64,
    per_page: i64,
}

// ── Handler ──────────────────────────────────────────────────────────────

pub struct ProductGetAllHandler {
    product_repo: Arc<dyn ProductRepo>,
}

impl ProductGetAllHandler {
    pub fn new(product_repo: Arc<dyn ProductRepo>) -> Self {
        Self { product_repo }
    }

    pub async fn execute(
        &self,
        query: ProductGetAllQuery,
    ) -> Result<(Vec<Product>, i64), ProductAppError> {
        let products = self.product_repo.get_all(query.page, query.per_page).await?;
        Ok(products)
    }
}

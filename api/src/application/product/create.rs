use std::sync::Arc;
use crate::domain::product::{
    PriceCents, Product, ProductDomainError, ProductName, ProductRepository, UnitAmount, UnitType,
};

pub struct CreateProductUseCase {
    product_repo: Arc<dyn ProductRepository>,
}

impl CreateProductUseCase {
    pub fn new(product_repo: Arc<dyn ProductRepository>) -> Self {
        Self { product_repo }
    }

    pub async fn execute(
        &self,
        name:                String,
        description:         Option<String>,
        price_cents:         i32,
        cost_price:          Option<i32>,
        unit_amount:         String,
        unit_type:           i32,
        stock:               i32,
        low_stock_threshold: i32,
        image_url:           Option<String>,
    ) -> Result<Product, ProductDomainError> {
        let product = Product::create(
            ProductName::new(name)?,
            description,
            PriceCents::new(price_cents)?,
            cost_price,
            UnitAmount::new(unit_amount)?,
            UnitType::new(unit_type),
            stock,
            low_stock_threshold,
            image_url,
        );
        self.product_repo.save(&product).await?;
        Ok(product)
    }
}

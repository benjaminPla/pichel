use crate::{
    application::product::errors::ProductAppError,
    domain::product::{
        ports::repository::ProductRepo,
        value_objects::{
            description::Description,
            name::Name,
            price_cents::PriceCents,
            sale_mode::SaleMode,
            symbol::Symbol,
            unit_of_measure::UnitOfMeasure,
        },
        Product,
    },
};
use std::sync::Arc;
use uuid::Uuid;

pub struct CreateProductInput {
    pub description: Option<String>,
    pub image_url:   Option<String>,
    pub name:        String,
    pub price_cents: u32,
    pub sale_mode:   String,
    pub symbols:     Vec<String>,
    pub updated_by:  Uuid,
}

pub struct CreateProductUseCase {
    product_repo: Arc<dyn ProductRepo>,
}

impl CreateProductUseCase {
    pub fn new(product_repo: Arc<dyn ProductRepo>) -> Self {
        Self { product_repo }
    }

    pub async fn execute(&self, input: CreateProductInput) -> Result<Product, ProductAppError> {
        let description     = input.description.map(Description::new).transpose()?;
        let name            = Name::new(input.name)?;
        let price_cents     = PriceCents::new(input.price_cents)?;
        let sale_mode       = input.sale_mode.parse::<SaleMode>()?;
        let symbols         = input.symbols.iter().map(|s| s.parse::<Symbol>()).collect::<Result<Vec<_>, _>>()?;
        let unit_of_measure = match &sale_mode {
            SaleMode::Unit => UnitOfMeasure::Unit,
            SaleMode::Bulk    => UnitOfMeasure::Kilogram,
        };
        let product = Product::new(description, input.image_url, name, price_cents, sale_mode, symbols, unit_of_measure);
        let product = self.product_repo.create(&product, input.updated_by).await?;
        Ok(product)
    }
}

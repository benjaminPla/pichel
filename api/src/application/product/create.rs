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

pub struct CreateProductInput {
    pub description:     Option<String>,
    pub image_url:       Option<String>,
    pub name:            String,
    pub price_cents:     u32,
    pub sale_mode:       String,
    pub symbols:         Vec<String>,
    pub unit_of_measure: String,
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
        let unit_of_measure = input.unit_of_measure.parse::<UnitOfMeasure>()?;
        let product         = Product::new(
            description,
            input.image_url,
            name,
            price_cents,
            sale_mode,
            symbols,
            unit_of_measure,
        );
        let product = self.product_repo.create(&product).await?;
        Ok(product)
    }
}

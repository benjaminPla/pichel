use std::sync::Arc;

use crate::{
    application::product::errors::ProductAppError,
    domain::{
        product::{
            aggregate_root::Product,
            repository::ProductRepo,
            value_objects::{
                description::Description, name::Name, symbol::Symbol,
                unit_of_measure::UnitOfMeasure,
            },
        },
    },
};

// ── Command ──────────────────────────────────────────────────────────────

pub struct ProductCreateCommand {
    pub description:         Option<String>,
    pub image_url:           Option<String>,
    pub low_stock_threshold: u32,
    pub name:                String,
    pub price_cents:         u32,
    pub stock:               u32,
    pub symbols:             Vec<String>,
    pub unit_of_measure:     String,
}

// ── Handler ──────────────────────────────────────────────────────────────

pub struct ProductCreateHandler {
    product_repo: Arc<dyn ProductRepo>,
}

impl ProductCreateHandler {
    pub fn new(product_repo: Arc<dyn ProductRepo>) -> Self {
        Self { product_repo }
    }

    pub async fn execute(&self, cmd: ProductCreateCommand) -> Result<Product, ProductAppError> {
        let description     = cmd.description.map(Description::new).transpose()?;
        let name            = Name::new(cmd.name)?;
        let symbols         = cmd.symbols.iter().map(|s| s.parse::<Symbol>()).collect::<Result<Vec<_>, _>>()?;
        let unit_of_measure = cmd.unit_of_measure.parse::<UnitOfMeasure>()?;
        let product         = Product::new(
            description,
            cmd.image_url,
            cmd.low_stock_threshold,
            name,
            cmd.price_cents,
            cmd.stock,
            symbols,
            unit_of_measure,
        );
        let product = self.product_repo.save(&product).await?;
        Ok(product)
    }
}

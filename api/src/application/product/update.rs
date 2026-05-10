use crate::{
    application::product::errors::ProductAppError,
    domain::product::{
        ports::repository::ProductRepo,
        value_objects::{
            description::Description, id::ProductId, name::Name, symbol::Symbol,
            unit_of_measure::UnitOfMeasure,
        },
        Product,
    },
};
use std::sync::Arc;
use uuid::Uuid;

pub struct UpdateProductInput {
    pub description:         Option<String>,
    pub id:                  Uuid,
    pub image_url:           Option<String>,
    pub low_stock_threshold: Option<u32>,
    pub name:                Option<String>,
    pub price_cents:         Option<u32>,
    pub stock:               Option<u32>,
    pub symbols:             Option<Vec<String>>,
    pub unit_of_measure:     Option<String>,
}

pub struct UpdateProductUseCase {
    product_repo: Arc<dyn ProductRepo>,
}

impl UpdateProductUseCase {
    pub fn new(product_repo: Arc<dyn ProductRepo>) -> Self {
        Self { product_repo }
    }

    pub async fn execute(&self, input: UpdateProductInput) -> Result<Product, ProductAppError> {
        let product_id          = ProductId::reconstitute(input.id);
        let current             = self.product_repo.get_by_id(&product_id).await?;
        let description         = match input.description {
            Some(d) => Some(Description::new(d)?),
            None    => current.get_description().cloned(),
        };
        let image_url           = input.image_url.or_else(|| current.get_image_url().map(str::to_string));
        let low_stock_threshold = input.low_stock_threshold.unwrap_or(current.get_low_stock_threshold());
        let name                = match input.name {
            Some(n) => Name::new(n)?,
            None    => current.get_name().clone(),
        };
        let price_cents         = input.price_cents.unwrap_or(current.get_price_cents());
        let stock               = input.stock.unwrap_or(current.get_stock());
        let symbols             = match input.symbols {
            Some(s) => s.iter().map(|s| s.parse::<Symbol>()).collect::<Result<Vec<_>, _>>()?,
            None    => current.get_symbols().to_vec(),
        };
        let unit_of_measure     = match input.unit_of_measure {
            Some(u) => u.parse::<UnitOfMeasure>()?,
            None    => current.get_unit_of_measure().clone(),
        };
        let updated = Product::reconstitute(
            description,
            current.get_id().clone(),
            image_url,
            low_stock_threshold,
            name,
            price_cents,
            stock,
            symbols,
            unit_of_measure,
        );
        let product = self.product_repo.update(&updated).await?;
        Ok(product)
    }
}

use crate::{
    application::product::errors::ProductAppError,
    domain::product::{
        ports::repository::ProductRepo,
        value_objects::{
            description::Description,
            id::ProductId,
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

pub struct UpdateProductInput {
    pub description: Option<String>,
    pub id:          Uuid,
    pub image_url:   Option<String>,
    pub name:        Option<String>,
    pub price_cents: Option<u32>,
    pub sale_mode:   Option<String>,
    pub symbols:     Option<Vec<String>>,
    pub updated_by:  Uuid,
}

pub struct UpdateProductUseCase {
    product_repo: Arc<dyn ProductRepo>,
}

impl UpdateProductUseCase {
    pub fn new(product_repo: Arc<dyn ProductRepo>) -> Self {
        Self { product_repo }
    }

    pub async fn execute(&self, input: UpdateProductInput) -> Result<Product, ProductAppError> {
        let product_id      = ProductId::reconstitute(input.id);
        let current         = self.product_repo.get_by_id(&product_id).await?;
        let description     = match input.description {
            Some(d) => Some(Description::new(d)?),
            None    => current.get_description().cloned(),
        };
        let image_url       = input.image_url.or_else(|| current.get_image_url().map(str::to_string));
        let name            = match input.name {
            Some(n) => Name::new(n)?,
            None    => current.get_name().clone(),
        };
        let price_cents     = match input.price_cents {
            Some(v) => PriceCents::new(v)?,
            None    => current.get_price_cents(),
        };
        let sale_mode       = match input.sale_mode {
            Some(s) => s.parse::<SaleMode>()?,
            None    => current.get_sale_mode().clone(),
        };
        let unit_of_measure = match &sale_mode {
            SaleMode::Unit => UnitOfMeasure::Unit,
            SaleMode::Bulk    => UnitOfMeasure::Kilogram,
        };
        let symbols         = match input.symbols {
            Some(s) => s.iter().map(|s| s.parse::<Symbol>()).collect::<Result<Vec<_>, _>>()?,
            None    => current.get_symbols().to_vec(),
        };
        let updated = Product::reconstitute(
            description,
            current.get_id().clone(),
            image_url,
            name,
            price_cents,
            sale_mode,
            symbols,
            unit_of_measure,
        );
        let product = self.product_repo.update(&updated, input.updated_by).await?;
        Ok(product)
    }
}

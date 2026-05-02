use std::sync::Arc;
use uuid::Uuid;
use crate::{
    application::product::errors::ProductAppError,
    domain::product::{
        aggregate_root::Product,
        ports::repository::ProductRepo,
        value_objects::{
            description::Description, id::ProductId, name::Name, symbol::Symbol,
            unit_of_measure::UnitOfMeasure,
        },
    },
};

// ── Command ──────────────────────────────────────────────────────────────

pub struct ProductUpdateCommand {
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

// ── Handler ──────────────────────────────────────────────────────────────

pub struct ProductUpdateHandler {
    product_repo: Arc<dyn ProductRepo>,
}

impl ProductUpdateHandler {
    pub fn new(product_repo: Arc<dyn ProductRepo>) -> Self {
        Self { product_repo }
    }

    pub async fn execute(&self, cmd: ProductUpdateCommand) -> Result<Product, ProductAppError> {
        let product_id          = ProductId::reconstitute(cmd.id);
        let current             = self.product_repo.get_by_id(&product_id).await?;
        let description         = match cmd.description {
            Some(d) => Some(Description::new(d)?),
            None    => current.get_description().cloned(),
        };
        let image_url           = cmd.image_url.or_else(|| current.get_image_url().map(str::to_string));
        let low_stock_threshold = cmd.low_stock_threshold.unwrap_or(current.get_low_stock_threshold());
        let name                = match cmd.name {
            Some(n) => Name::new(n)?,
            None    => current.get_name().clone(),
        };
        let price_cents         = cmd.price_cents.unwrap_or(current.get_price_cents());
        let stock               = cmd.stock.unwrap_or(current.get_stock());
        let symbols             = match cmd.symbols {
            Some(s) => s.iter().map(|s| s.parse::<Symbol>()).collect::<Result<Vec<_>, _>>()?,
            None    => current.get_symbols().to_vec(),
        };
        let unit_of_measure     = match cmd.unit_of_measure {
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

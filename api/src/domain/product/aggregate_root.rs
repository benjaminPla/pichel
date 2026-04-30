use super::{
    derived_values::product_stock_status::StockStatus,
    value_objects::{
        id::ProductId,
        description::Description,
        name::Name,
        symbol::Symbol,
        unit_of_measure::{UnitOfMeasure, UnitOfMeasureError},
    }
};

#[derive(Debug, Clone)]
pub struct Product {
    description:         Option<Description>,
    id:                  ProductId,
    image_url:           Option<String>,
    low_stock_threshold: u32,
    name:                Name,
    price_cents:         u32,
    stock:               u32,
    symbols:             Vec<Symbol>,
    unit_of_measure:     UnitOfMeasure,
}

impl Product {
    pub fn compute_stock_status(&self) -> StockStatus { StockStatus::from(self) }

    pub fn new(
        description:         Option<Description>,
        image_url:           Option<String>,
        low_stock_threshold: u32,
        name:                Name,
        price_cents:         u32,
        stock:               u32,
        symbols:             Vec<Symbol>,
        unit_of_measure:     UnitOfMeasure,
    ) -> Self {
        let id  = ProductId::new();
        Self {
            description,
            id,
            image_url,
            low_stock_threshold,
            name,
            price_cents,
            stock,
            symbols,
            unit_of_measure,
        }
    }

    pub fn price_for(&self, amount: f64, unit: &UnitOfMeasure) -> Result<u32, UnitOfMeasureError> {
        if unit.dimension() != self.unit_of_measure.dimension() {
            return Err(UnitOfMeasureError::Invalid(format!(
                "cannot convert {:?} to {:?}",
                unit.dimension(),
                self.unit_of_measure.dimension()
            )));
        }
        let order_base   = amount * unit.to_base_factor();
        let product_base = self.unit_of_measure.to_base_factor();
        let ratio        = order_base / product_base;
        Ok((ratio * self.price_cents as f64).round() as u32)
    }

    pub fn reconstitute(
        description:         Option<Description>,
        id:                  ProductId,
        image_url:           Option<String>,
        low_stock_threshold: u32,
        name:                Name,
        price_cents:         u32,
        stock:               u32,
        symbols:             Vec<Symbol>,
        unit_of_measure:     UnitOfMeasure,
    ) -> Self {
        Self {
            description,
            id,
            image_url,
            low_stock_threshold,
            name,
            price_cents,
            stock,
            symbols,
            unit_of_measure,
        }
    }

    // ── Getters ──────────────────────────────────────────────────────────────

    pub fn get_description(&self)         -> Option<&Description> { self.description.as_ref() }
    pub fn get_id(&self)                  -> &ProductId           { &self.id }
    pub fn get_image_url(&self)           -> Option<&str>         { self.image_url.as_deref() }
    pub fn get_low_stock_threshold(&self) -> u32                  { self.low_stock_threshold }
    pub fn get_name(&self)                -> &Name                { &self.name }
    pub fn get_price_cents(&self)         -> u32                  { self.price_cents }
    pub fn get_stock(&self)               -> u32                  { self.stock }
    pub fn get_symbols(&self)             -> &[Symbol]            { &self.symbols }
    pub fn get_unit_of_measure(&self)     -> &UnitOfMeasure       { &self.unit_of_measure }
}

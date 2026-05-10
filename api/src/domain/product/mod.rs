pub mod derived_values;
pub mod ports;
pub mod value_objects;

use {
    derived_values::stock_status::StockStatus,
    value_objects::{
        description::Description,
        id::ProductId,
        name::Name,
        price_cents::PriceCents,
        sale_mode::SaleMode,
        stock::Stock,
        symbol::Symbol,
        unit_of_measure::{UnitOfMeasure, UnitOfMeasureError},
    },
};

#[derive(Debug, Clone)]
pub struct Product {
    description:     Option<Description>,
    id:              ProductId,
    image_url:       Option<String>,
    name:            Name,
    price_cents:     PriceCents,
    sale_mode:       SaleMode,
    stock:           Stock,
    symbols:         Vec<Symbol>,
    unit_of_measure: UnitOfMeasure,
}

impl Product {
    pub fn compute_stock_status(&self) -> StockStatus { StockStatus::from(self) }

    pub fn new(
        description:     Option<Description>,
        image_url:       Option<String>,
        name:            Name,
        price_cents:     PriceCents,
        sale_mode:       SaleMode,
        stock:           Stock,
        symbols:         Vec<Symbol>,
        unit_of_measure: UnitOfMeasure,
    ) -> Self {
        Self {
            description,
            id: ProductId::new(),
            image_url,
            name,
            price_cents,
            sale_mode,
            stock,
            symbols,
            unit_of_measure,
        }
    }

    pub fn reconstitute(
        description:     Option<Description>,
        id:              ProductId,
        image_url:       Option<String>,
        name:            Name,
        price_cents:     PriceCents,
        sale_mode:       SaleMode,
        stock:           Stock,
        symbols:         Vec<Symbol>,
        unit_of_measure: UnitOfMeasure,
    ) -> Self {
        Self { description, id, image_url, name, price_cents, sale_mode, stock, symbols, unit_of_measure }
    }

    pub fn price_for(&self, amount: f64, unit: &UnitOfMeasure) -> Result<PriceCents, UnitOfMeasureError> {
        if unit.dimension() != self.unit_of_measure.dimension() {
            return Err(UnitOfMeasureError::Invalid(format!(
                "cannot convert {:?} to {:?}",
                unit.dimension(),
                self.unit_of_measure.dimension()
            )));
        }
        let ratio = (amount * unit.to_base_factor()) / self.unit_of_measure.to_base_factor();
        Ok(PriceCents::reconstitute((ratio * self.price_cents.value() as f64).round() as u32))
    }

    // ── Getters ──────────────────────────────────────────────────────────────

    pub fn get_description(&self)     -> Option<&Description> { self.description.as_ref() }
    pub fn get_id(&self)              -> &ProductId           { &self.id }
    pub fn get_image_url(&self)       -> Option<&str>         { self.image_url.as_deref() }
    pub fn get_name(&self)            -> &Name                { &self.name }
    pub fn get_price_cents(&self)     -> PriceCents           { self.price_cents }
    pub fn get_sale_mode(&self)       -> &SaleMode            { &self.sale_mode }
    pub fn get_stock(&self)           -> Stock                { self.stock }
    pub fn get_symbols(&self)         -> &[Symbol]            { &self.symbols }
    pub fn get_unit_of_measure(&self) -> &UnitOfMeasure       { &self.unit_of_measure }
}

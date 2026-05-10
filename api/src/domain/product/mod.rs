pub mod ports;
pub mod value_objects;

use value_objects::{
    description::Description,
    id::ProductId,
    name::Name,
    price_cents::PriceCents,
    sale_mode::SaleMode,
    symbol::Symbol,
    unit_of_measure::UnitOfMeasure,
};

#[derive(Debug, Clone)]
pub struct Product {
    description:     Option<Description>,
    id:              ProductId,
    image_url:       Option<String>,
    name:            Name,
    price_cents:     PriceCents,
    sale_mode:       SaleMode,
    symbols:         Vec<Symbol>,
    unit_of_measure: UnitOfMeasure,
}

impl Product {
    pub fn new(
        description:     Option<Description>,
        image_url:       Option<String>,
        name:            Name,
        price_cents:     PriceCents,
        sale_mode:       SaleMode,
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
        symbols:         Vec<Symbol>,
        unit_of_measure: UnitOfMeasure,
    ) -> Self {
        Self { description, id, image_url, name, price_cents, sale_mode, symbols, unit_of_measure }
    }

    // ── Getters ──────────────────────────────────────────────────────────────

    pub fn get_description(&self)     -> Option<&Description> { self.description.as_ref() }
    pub fn get_id(&self)              -> &ProductId           { &self.id }
    pub fn get_image_url(&self)       -> Option<&str>         { self.image_url.as_deref() }
    pub fn get_name(&self)            -> &Name                { &self.name }
    pub fn get_price_cents(&self)     -> PriceCents           { self.price_cents }
    pub fn get_sale_mode(&self)       -> &SaleMode            { &self.sale_mode }
    pub fn get_symbols(&self)         -> &[Symbol]            { &self.symbols }
    pub fn get_unit_of_measure(&self) -> &UnitOfMeasure       { &self.unit_of_measure }
}

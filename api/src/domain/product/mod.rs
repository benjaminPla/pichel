pub mod ports;
pub mod value_objects;

use value_objects::{
    category_summary::CategorySummary,
    description::Description,
    id::ProductId,
    name::Name,
    plu::Plu,
    price_cents::PriceCents,
    sale_mode::SaleMode,
    symbol::Symbol,
    unit_of_measure::UnitOfMeasure,
};

#[derive(Debug, Clone)]
pub struct Product {
    categories:      Vec<CategorySummary>,
    description:     Option<Description>,
    id:              ProductId,
    active:          bool,
    image_url:       Option<String>,
    name:            Name,
    plu:             Option<Plu>,
    price_cents:     PriceCents,
    sale_mode:       SaleMode,
    symbols:         Vec<Symbol>,
    unit_of_measure: UnitOfMeasure,
}

impl Product {
    pub fn new(
        active:          bool,
        description:     Option<Description>,
        image_url:       Option<String>,
        name:            Name,
        price_cents:     PriceCents,
        sale_mode:       SaleMode,
        symbols:         Vec<Symbol>,
        unit_of_measure: UnitOfMeasure,
    ) -> Self {
        Self {
            categories: Vec::new(),
            description,
            id: ProductId::new(),
            active,
            image_url,
            name,
            plu: None,
            price_cents,
            sale_mode,
            symbols,
            unit_of_measure,
        }
    }

    pub fn reconstitute(
        categories:      Vec<CategorySummary>,
        description:     Option<Description>,
        id:              ProductId,
        active:          bool,
        image_url:       Option<String>,
        name:            Name,
        plu:             Plu,
        price_cents:     PriceCents,
        sale_mode:       SaleMode,
        symbols:         Vec<Symbol>,
        unit_of_measure: UnitOfMeasure,
    ) -> Self {
        Self { categories, description, id, active, image_url, name, plu: Some(plu), price_cents, sale_mode, symbols, unit_of_measure }
    }

    /// Attaches the resolved category id/name pairs after a create/update round-trip
    /// (the join-table write happens separately from the `products` row write).
    pub fn with_categories(mut self, categories: Vec<CategorySummary>) -> Self {
        self.categories = categories;
        self
    }

    // ── Getters ──────────────────────────────────────────────────────────────

    pub fn get_categories(&self)      -> &[CategorySummary]   { &self.categories }
    pub fn get_description(&self)     -> Option<&Description> { self.description.as_ref() }
    pub fn get_id(&self)              -> &ProductId           { &self.id }
    pub fn get_active(&self)          -> bool                 { self.active }
    pub fn get_image_url(&self)       -> Option<&str>         { self.image_url.as_deref() }
    pub fn get_name(&self)            -> &Name                { &self.name }
    pub fn get_plu(&self)             -> Option<Plu>          { self.plu }
    pub fn get_price_cents(&self)     -> PriceCents           { self.price_cents }
    pub fn get_sale_mode(&self)       -> &SaleMode            { &self.sale_mode }
    pub fn get_symbols(&self)         -> &[Symbol]            { &self.symbols }
    pub fn get_unit_of_measure(&self) -> &UnitOfMeasure       { &self.unit_of_measure }
}

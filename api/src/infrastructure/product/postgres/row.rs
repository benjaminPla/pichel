use uuid::Uuid;

use crate::domain::product::{
    aggregate_root::Product,
    ports::repository::ProductRepoError,
    value_objects::{
        description::Description, id::ProductId, name::Name, symbol::Symbol,
        unit_of_measure::UnitOfMeasure,
    },
};

#[derive(sqlx::FromRow)]
pub struct ProductRow {
    description: Option<String>,
    id: Uuid,
    image_url: Option<String>,
    low_stock_threshold: i32,
    name: String,
    price_cents: i32,
    stock: i32,
    symbols: Vec<String>,
    unit_of_measure: String,
}

impl TryFrom<ProductRow> for Product {
    type Error = ProductRepoError;

    fn try_from(r: ProductRow) -> Result<Product, ProductRepoError> {
        let description         = r.description.map(|d| Description::new(d)).transpose()?;
        let id                  = ProductId::reconstitute(r.id);
        let low_stock_threshold = u32::try_from(r.low_stock_threshold)?;
        let name                = Name::new(r.name)?;
        let price_cents         = u32::try_from(r.price_cents)?;
        let stock               = u32::try_from(r.stock)?;
        let symbols             = r.symbols.iter().map(|s| s.parse::<Symbol>()).collect::<Result<Vec<_>, _>>()?;
        let unit_of_measure     = r.unit_of_measure.parse::<UnitOfMeasure>()?;
        let product             = Product::reconstitute(description, id, r.image_url, low_stock_threshold, name, price_cents, stock, symbols, unit_of_measure);
        Ok(product)
    }
}

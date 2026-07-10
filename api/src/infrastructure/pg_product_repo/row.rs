use uuid::Uuid;
use crate::domain::product::{
    ports::repository::ProductRepoError,
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
};

#[derive(sqlx::FromRow)]
pub struct ProductRow {
    description:     Option<String>,
    id:              Uuid,
    active:          bool,
    image_url:       Option<String>,
    name:            String,
    price_cents:     i32,
    sale_mode:       String,
    symbols:         Vec<String>,
    unit_of_measure: String,
}

impl TryFrom<ProductRow> for Product {
    type Error = ProductRepoError;

    fn try_from(r: ProductRow) -> Result<Product, ProductRepoError> {
        let description     = r.description.map(Description::new).transpose()?;
        let id              = ProductId::reconstitute(r.id);
        let name            = Name::new(r.name)?;
        let price_cents     = PriceCents::new(u32::try_from(r.price_cents)?)?;
        let sale_mode       = r.sale_mode.parse::<SaleMode>()?;
        let symbols         = r.symbols.iter().map(|s| s.parse::<Symbol>()).collect::<Result<Vec<_>, _>>()?;
        let unit_of_measure = r.unit_of_measure.parse::<UnitOfMeasure>()?;
        Ok(Product::reconstitute(description, id, r.active, r.image_url, name, price_cents, sale_mode, symbols, unit_of_measure))
    }
}

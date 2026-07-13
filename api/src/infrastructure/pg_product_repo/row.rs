use uuid::Uuid;
use crate::domain::product::{
    ports::repository::ProductRepoError,
    value_objects::{
        category_summary::CategorySummary,
        description::Description,
        id::ProductId,
        name::Name,
        plu::Plu,
        price_cents::PriceCents,
        sale_mode::SaleMode,
        symbol::Symbol,
        unit_of_measure::UnitOfMeasure,
    },
    Product,
};

#[derive(sqlx::FromRow)]
pub struct ProductRow {
    category_ids:    Vec<Uuid>,
    category_names:  Vec<String>,
    description:     Option<String>,
    id:              Uuid,
    active:          bool,
    image_url:       Option<String>,
    name:            String,
    plu:             i16,
    price_cents:     i32,
    sale_mode:       String,
    symbols:         Vec<String>,
    unit_of_measure: String,
}

/// Minimal read model for `categories` rows, used to attach id/name pairs to a `Product`
/// after a create/update write to the `product_categories` join table.
#[derive(sqlx::FromRow)]
pub struct CategoryRow {
    pub id:   Uuid,
    pub name: String,
}

impl TryFrom<ProductRow> for Product {
    type Error = ProductRepoError;

    fn try_from(r: ProductRow) -> Result<Product, ProductRepoError> {
        let categories = r.category_ids.into_iter().zip(r.category_names)
            .map(|(id, name)| CategorySummary { id, name })
            .collect::<Vec<_>>();
        let description     = r.description.map(Description::new).transpose()?;
        let id              = ProductId::reconstitute(r.id);
        let name            = Name::new(r.name)?;
        let plu             = Plu::new(u16::try_from(r.plu)?)?;
        let price_cents     = PriceCents::new(u32::try_from(r.price_cents)?)?;
        let sale_mode       = r.sale_mode.parse::<SaleMode>()?;
        let symbols         = r.symbols.iter().map(|s| s.parse::<Symbol>()).collect::<Result<Vec<_>, _>>()?;
        let unit_of_measure = r.unit_of_measure.parse::<UnitOfMeasure>()?;
        Ok(Product::reconstitute(categories, description, id, r.active, r.image_url, name, plu, price_cents, sale_mode, symbols, unit_of_measure))
    }
}

use crate::domain::shared::value_objects::ids::EntityId;

#[derive(Debug, Clone)]
pub struct ProductMarker;
pub type ProductId = EntityId<ProductMarker>;

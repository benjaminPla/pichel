use crate::domain::shared::value_objects::ids::EntityId;

#[derive(Debug, Clone)]
pub struct CategoryMarker;
pub type CategoryId = EntityId<CategoryMarker>;

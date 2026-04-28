use crate::domain::shared::value_objects::ids::EntityId;

#[derive(Debug, Clone)]
pub struct UserMarker;
pub type UserId = EntityId<UserMarker>;

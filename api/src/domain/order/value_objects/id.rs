use crate::domain::shared::value_objects::ids::EntityId;

#[derive(Debug, Clone)]
pub struct OrderMarker;
pub type OrderId = EntityId<OrderMarker>;

#[derive(Debug, Clone)]
pub struct OrderItemMarker;
pub type OrderItemId = EntityId<OrderItemMarker>;

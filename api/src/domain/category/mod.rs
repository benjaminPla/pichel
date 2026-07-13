pub mod ports;
pub mod value_objects;

use value_objects::{id::CategoryId, name::Name};

#[derive(Debug, Clone)]
pub struct Category {
    id:   CategoryId,
    name: Name,
}

impl Category {
    pub fn new(name: Name) -> Self {
        Self { id: CategoryId::new(), name }
    }

    pub fn reconstitute(id: CategoryId, name: Name) -> Self {
        Self { id, name }
    }

    // ── Getters ──────────────────────────────────────────────────────────────

    pub fn get_id(&self)   -> &CategoryId { &self.id }
    pub fn get_name(&self) -> &Name       { &self.name }
}

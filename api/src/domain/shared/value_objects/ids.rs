use std::marker::PhantomData;

use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EntityId<T> {
    value: Uuid,
    _marker: PhantomData<T>,
}

impl<T> EntityId<T> {
    pub fn new() -> Self {
        Self { value: Uuid::new_v4(), _marker: PhantomData }
    }

    pub fn reconstitute(value: Uuid) -> Self {
        Self { value, _marker: PhantomData }
    }

    pub fn value(&self) -> Uuid { self.value }
}

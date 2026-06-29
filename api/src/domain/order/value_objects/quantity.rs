#[derive(Debug, Clone, Copy)]
pub struct Quantity(u32);

impl Quantity {
    pub fn new(value: u32) -> Result<Self, QuantityError> {
        if value == 0 { return Err(QuantityError::Zero); }
        Ok(Self(value))
    }

    pub fn value(&self) -> u32 { self.0 }
}

#[derive(Debug, thiserror::Error)]
pub enum QuantityError {
    #[error("quantity must be greater than zero")]
    Zero,
}

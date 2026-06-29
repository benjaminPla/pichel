#[derive(Debug, Clone, Copy)]
pub struct PriceCents(u32);

impl PriceCents {
    pub fn new(value: u32) -> Result<Self, PriceCentsError> {
        if value == 0 { return Err(PriceCentsError::Zero); }
        Ok(Self(value))
    }

    pub fn value(&self) -> u32 { self.0 }
}

#[derive(Debug, thiserror::Error)]
pub enum PriceCentsError {
    #[error("price must be greater than zero")]
    Zero,
}

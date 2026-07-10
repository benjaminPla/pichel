#[derive(Debug, Clone, Copy)]
pub struct Plu(u16);

impl Plu {
    const MIN: u16 = 1;
    const MAX: u16 = 800;

    pub fn new(value: u16) -> Result<Self, PluError> {
        if value < Self::MIN || value > Self::MAX {
            return Err(PluError::OutOfRange(Self::MIN, Self::MAX));
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> u16 { self.0 }
}

#[derive(Debug, thiserror::Error)]
pub enum PluError {
    #[error("plu must be between {0} and {1}")]
    OutOfRange(u16, u16),
}

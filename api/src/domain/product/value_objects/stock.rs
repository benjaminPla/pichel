#[derive(Debug, Clone, Copy)]
pub struct Stock(u32);

impl Stock {
    pub fn new(value: u32) -> Self { Self(value) }
    pub fn value(&self) -> u32     { self.0 }
}

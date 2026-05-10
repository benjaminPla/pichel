use crate::domain::product::Product;

#[derive(Debug, Clone, PartialEq)]
pub enum StockStatus {
    InStock,
    OutOfStock,
}

impl StockStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::InStock    => "in_stock",
            Self::OutOfStock => "out_of_stock",
        }
    }
}

impl From<&Product> for StockStatus {
    fn from(p: &Product) -> Self {
        if p.get_stock().value() == 0 { Self::OutOfStock } else { Self::InStock }
    }
}

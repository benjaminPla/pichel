use crate::domain::product::Product;

#[derive(Debug, Clone, PartialEq)]
pub enum StockStatus {
    InStock,
    LowStock,
    OutOfStock,
}

impl StockStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::InStock    => "in_stock",
            Self::LowStock   => "low_stock",
            Self::OutOfStock => "out_of_stock",
        }
    }
}

impl From<&Product> for StockStatus {
    fn from(p: &Product) -> Self {
        let stock               = p.get_stock();
        let low_stock_threshold = p.get_low_stock_threshold();
        if stock == 0 {
            StockStatus::OutOfStock
        } else if stock <= low_stock_threshold {
            StockStatus::LowStock
        } else {
            StockStatus::InStock
        }
    }
}

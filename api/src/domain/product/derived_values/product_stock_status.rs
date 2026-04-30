use crate::domain::product::aggregate_root::Product;

#[derive(Debug, Clone, PartialEq)]
pub enum StockStatus {
    InStock,
    LowStock,
    OutOfStock,
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

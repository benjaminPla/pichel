pub mod product;
pub mod repository;

pub use product::{Price, Product, ProductDomainError, ProductId, ProductName};
pub use repository::ProductRepository;

/// Product catalog module with CRUD operations, inventory management, and filtering
pub mod catalog;

// Re-export main types for convenience
pub use catalog::{NewProduct, Product, ProductFilter, ProductService};

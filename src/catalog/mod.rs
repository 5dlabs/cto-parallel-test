/// Product catalog models
pub mod models;

/// Product catalog service with CRUD operations
pub mod service;

// Re-export commonly used types for convenience
pub use models::{NewProduct, Product, ProductFilter};
pub use service::ProductService;

pub mod models;
pub mod service;

// Public API exports for use by other tasks (Task 5 cart routes, Task 7 integration tests)
#[allow(unused_imports)] // Used by cart module and tests
pub use self::models::Product;
pub use self::service::ProductService;

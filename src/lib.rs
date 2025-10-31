//! CTO Parallel Test - E-commerce API Library
//!
//! This library provides core functionality for the e-commerce test API,
//! including product catalog management.

pub mod catalog;

// Re-export key types for convenience
pub use catalog::{Product, ProductService};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

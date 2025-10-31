//! CTO Parallel Test - E-commerce API
//!
//! This library provides a product catalog module with in-memory storage,
//! CRUD operations, filtering, and inventory management.

pub mod catalog;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

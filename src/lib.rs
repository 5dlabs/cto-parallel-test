//! CTO Parallel Test - E-commerce API
//!
//! This library provides database schema definitions for a test e-commerce application.

pub mod api;
pub mod auth;
pub mod cart;
pub mod catalog;
pub mod schema;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

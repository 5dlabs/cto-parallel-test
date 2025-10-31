//! E-commerce API Library
//!
//! This library provides database schema definitions, authentication, and models
//! for an e-commerce application.

pub mod auth;
pub mod schema;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

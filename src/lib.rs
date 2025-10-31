// E-commerce API Library
//
// This library provides the database schema and core data models
// for a test e-commerce API built with Diesel ORM.

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

pub mod jwt;
pub mod models;

pub use self::jwt::{create_token, validate_token, Claims};
pub use self::models::User;

// Keep tests colocated for easy discovery
#[cfg(test)]
mod tests;

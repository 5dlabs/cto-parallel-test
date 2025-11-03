//! Common test utilities shared across integration tests
//!
//! Provides helper functions for creating test data and initializing services.

use cto_parallel_test::auth::models::User;
use cto_parallel_test::catalog::models::NewProduct;
use cto_parallel_test::catalog::service::ProductService;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use std::str::FromStr;

/// Creates a test user with the given ID and credentials
///
/// # Arguments
///
/// * `id` - User ID
/// * `username` - Username
/// * `password` - Plaintext password (will be hashed)
///
/// # Returns
///
/// User struct with hashed password
#[must_use]
pub fn create_test_user(id: i32, username: &str, password: &str) -> User {
    let password_hash = User::hash_password(password);
    User {
        id,
        username: username.to_string(),
        email: format!("{username}@example.com"),
        password_hash,
    }
}

/// Creates a test product with the given parameters
///
/// # Arguments
///
/// * `name` - Product name
/// * `price` - Product price (will be converted to Decimal)
/// * `inventory` - Inventory count
///
/// # Returns
///
/// `NewProduct` struct ready for creation
#[must_use]
pub fn create_test_product(name: &str, price: f64, inventory: i32) -> NewProduct {
    NewProduct {
        name: name.to_string(),
        description: format!("Test description for {name}"),
        price: Decimal::from_f64(price).expect("Failed to convert price to Decimal"),
        inventory_count: inventory,
    }
}

/// Creates a product service initialized with test products
///
/// # Returns
///
/// `ProductService` with 3 pre-populated test products
#[must_use]
pub fn create_test_product_service() -> ProductService {
    let service = ProductService::new();

    // Create some test products
    let _ = service.create(create_test_product("Laptop", 999.99, 10));
    let _ = service.create(create_test_product("Mouse", 29.99, 50));
    let _ = service.create(create_test_product("Keyboard", 79.99, 25));

    service
}

/// Parses a string price into a Decimal
///
/// # Arguments
///
/// * `price` - Price as string (e.g., "19.99")
///
/// # Returns
///
/// Decimal representation of the price
///
/// # Panics
///
/// Panics if the string cannot be parsed as a Decimal
#[must_use]
pub fn decimal_from_str(price: &str) -> Decimal {
    Decimal::from_str(price).expect("Failed to parse decimal")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_test_user() {
        let user = create_test_user(1, "testuser", "password123");
        assert_eq!(user.id, 1);
        assert_eq!(user.username, "testuser");
        assert_eq!(user.email, "testuser@example.com");
        assert!(user.verify_password("password123"));
        assert!(!user.verify_password("wrong"));
    }

    #[test]
    fn test_create_test_product() {
        let product = create_test_product("Test Item", 49.99, 100);
        assert_eq!(product.name, "Test Item");
        assert_eq!(product.price, Decimal::from_f64(49.99).unwrap());
        assert_eq!(product.inventory_count, 100);
    }

    #[test]
    fn test_create_test_product_service() {
        let service = create_test_product_service();
        let products = service.get_all();
        assert_eq!(products.len(), 3);
        assert_eq!(products[0].name, "Laptop");
        assert_eq!(products[1].name, "Mouse");
        assert_eq!(products[2].name, "Keyboard");
    }

    #[test]
    fn test_decimal_from_str() {
        let price = decimal_from_str("19.99");
        assert_eq!(price.to_string(), "19.99");
    }
}

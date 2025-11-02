// Common test utilities for integration tests
#![allow(dead_code)] // Functions are used by different test files

use ecommerce_catalog::catalog::{NewProduct, ProductService};
use rust_decimal_macros::dec;

/// Creates a test product service with sample data
pub fn create_test_product_service() -> ProductService {
    let service = ProductService::new();

    // Add sample products for testing
    let _ = service.create(NewProduct {
        name: "Laptop".to_string(),
        description: "High-performance gaming laptop with RGB keyboard".to_string(),
        price: dec!(1299.99),
        inventory_count: 15,
    });

    let _ = service.create(NewProduct {
        name: "Wireless Mouse".to_string(),
        description: "Ergonomic wireless mouse with precision tracking".to_string(),
        price: dec!(29.99),
        inventory_count: 50,
    });

    let _ = service.create(NewProduct {
        name: "Mechanical Keyboard".to_string(),
        description: "RGB mechanical keyboard with Cherry MX switches".to_string(),
        price: dec!(149.99),
        inventory_count: 30,
    });

    let _ = service.create(NewProduct {
        name: "4K Monitor".to_string(),
        description: "27-inch 4K UHD monitor with HDR support".to_string(),
        price: dec!(599.99),
        inventory_count: 0, // Out of stock
    });

    let _ = service.create(NewProduct {
        name: "USB-C Hub".to_string(),
        description: "7-in-1 USB-C hub with HDMI and ethernet".to_string(),
        price: dec!(45.99),
        inventory_count: 100,
    });

    service
}

/// Creates a single test product for unit testing
#[must_use]
pub fn create_sample_product() -> NewProduct {
    NewProduct {
        name: "Test Product".to_string(),
        description: "A sample test product".to_string(),
        price: dec!(99.99),
        inventory_count: 10,
    }
}

/// Helper to create a test user password hash
#[must_use]
fn create_test_password_hash(password: &str) -> String {
    use ecommerce_catalog::auth::User;
    User::hash_password(password)
}

/// Helper to create a test user
#[must_use]
pub fn create_test_user(
    id: i32,
    username: &str,
    email: &str,
    password: &str,
) -> ecommerce_catalog::auth::User {
    ecommerce_catalog::auth::User {
        id,
        username: username.to_string(),
        email: email.to_string(),
        password_hash: create_test_password_hash(password),
    }
}

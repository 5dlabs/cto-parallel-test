// Unit tests for database models (no database connection required)

use ecommerce_api::models::{NewCart, NewCartItem, NewProduct, NewUser};

#[test]
fn test_new_user_creation() {
    let user = NewUser {
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password_hash: "hashed_password".to_string(),
    };

    assert_eq!(user.username, "testuser");
    assert_eq!(user.email, "test@example.com");
    assert_eq!(user.password_hash, "hashed_password");
}

#[test]
fn test_new_product_creation() {
    let product = NewProduct {
        name: "Test Product".to_string(),
        description: Some("A test product".to_string()),
        price: rust_decimal::Decimal::new(1999, 2), // $19.99
        inventory_count: 100,
    };

    assert_eq!(product.name, "Test Product");
    assert_eq!(product.description, Some("A test product".to_string()));
    assert_eq!(product.price.to_string(), "19.99");
    assert_eq!(product.inventory_count, 100);
}

#[test]
fn test_new_product_without_description() {
    let product = NewProduct {
        name: "Simple Product".to_string(),
        description: None,
        price: rust_decimal::Decimal::new(999, 2), // $9.99
        inventory_count: 50,
    };

    assert_eq!(product.name, "Simple Product");
    assert!(product.description.is_none());
    assert_eq!(product.price.to_string(), "9.99");
    assert_eq!(product.inventory_count, 50);
}

#[test]
fn test_new_cart_creation() {
    let cart = NewCart { user_id: 42 };
    assert_eq!(cart.user_id, 42);
}

#[test]
fn test_new_cart_item_creation() {
    let cart_item = NewCartItem {
        cart_id: 1,
        product_id: 2,
        quantity: 3,
    };

    assert_eq!(cart_item.cart_id, 1);
    assert_eq!(cart_item.product_id, 2);
    assert_eq!(cart_item.quantity, 3);
}

#[test]
fn test_decimal_precision() {
    // Test that Decimal maintains precision for financial calculations
    let price1 = rust_decimal::Decimal::new(1234, 2); // $12.34
    let price2 = rust_decimal::Decimal::new(5678, 2); // $56.78

    assert_eq!(price1.to_string(), "12.34");
    assert_eq!(price2.to_string(), "56.78");

    // Test addition maintains precision
    let total = price1 + price2;
    assert_eq!(total.to_string(), "69.12");
}

#[test]
fn test_decimal_multiplication() {
    // Test quantity * price calculation
    let price = rust_decimal::Decimal::new(2999, 2); // $29.99
    let quantity = rust_decimal::Decimal::from(3);

    let total = price * quantity;
    assert_eq!(total.to_string(), "89.97");
}

#[test]
fn test_large_decimal_values() {
    // Test that large prices are handled correctly
    let price = rust_decimal::Decimal::new(999_999, 2); // $9999.99
    assert_eq!(price.to_string(), "9999.99");
}

#[test]
fn test_zero_inventory() {
    let product = NewProduct {
        name: "Out of Stock Product".to_string(),
        description: Some("Currently unavailable".to_string()),
        price: rust_decimal::Decimal::new(1500, 2),
        inventory_count: 0,
    };

    assert_eq!(product.inventory_count, 0);
}

#[test]
fn test_negative_inventory_representation() {
    // While the database should enforce non-negative inventory,
    // the model can represent it
    let product = NewProduct {
        name: "Test Product".to_string(),
        description: None,
        price: rust_decimal::Decimal::new(1000, 2),
        inventory_count: -1,
    };

    assert_eq!(product.inventory_count, -1);
}

#[test]
fn test_user_fields_with_special_characters() {
    let user = NewUser {
        username: "test.user_123".to_string(),
        email: "test+tag@example.com".to_string(),
        password_hash: "$2b$12$abcdefghijklmnopqrstuv".to_string(),
    };

    assert!(user.username.contains('.'));
    assert!(user.username.contains('_'));
    assert!(user.email.contains('+'));
    assert!(user.password_hash.starts_with('$'));
}

#[test]
fn test_product_name_with_unicode() {
    let product = NewProduct {
        name: "Café ☕ Product".to_string(),
        description: Some("特殊字符 test".to_string()),
        price: rust_decimal::Decimal::new(500, 2),
        inventory_count: 10,
    };

    assert!(product.name.contains("Café"));
    assert!(product.name.contains("☕"));
    assert!(product.description.unwrap().contains("特殊字符"));
}

#[test]
fn test_models_implement_debug() {
    // Verify models implement Debug trait for better error messages
    let user = NewUser {
        username: "test".to_string(),
        email: "test@example.com".to_string(),
        password_hash: "hash".to_string(),
    };

    let debug_str = format!("{user:?}");
    assert!(debug_str.contains("NewUser"));
}

#[test]
fn test_models_implement_clone() {
    // Verify models implement Clone trait
    let original = NewProduct {
        name: "Original".to_string(),
        description: Some("Test".to_string()),
        price: rust_decimal::Decimal::new(100, 2),
        inventory_count: 5,
    };

    let cloned = original.clone();
    assert_eq!(original.name, cloned.name);
    assert_eq!(original.price, cloned.price);
}

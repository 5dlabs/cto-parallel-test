use crate::models::*;
use chrono::DateTime;

#[test]
fn test_user_model_creation() {
    let dt = DateTime::from_timestamp(0, 0).unwrap();
    let user = User {
        id: 1,
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password_hash: "hashed_password".to_string(),
        created_at: dt.naive_utc(),
    };
    assert_eq!(user.username, "testuser");
    assert_eq!(user.email, "test@example.com");
}

#[test]
fn test_new_user_creation() {
    let new_user = NewUser {
        username: "newuser".to_string(),
        email: "new@example.com".to_string(),
        password_hash: "hashed_password".to_string(),
    };
    assert_eq!(new_user.username, "newuser");
}

#[test]
fn test_product_model_creation() {
    let product = Product {
        id: 1,
        name: "Test Product".to_string(),
        description: Some("A test product".to_string()),
        price: bigdecimal::BigDecimal::from(100),
        inventory_count: 50,
    };
    assert_eq!(product.name, "Test Product");
    assert_eq!(product.inventory_count, 50);
}

#[test]
fn test_cart_model_creation() {
    let dt = DateTime::from_timestamp(0, 0).unwrap();
    let cart = Cart {
        id: 1,
        user_id: 1,
        created_at: dt.naive_utc(),
    };
    assert_eq!(cart.user_id, 1);
}

#[test]
fn test_cart_item_model_creation() {
    let cart_item = CartItem {
        id: 1,
        cart_id: 1,
        product_id: 1,
        quantity: 5,
    };
    assert_eq!(cart_item.quantity, 5);
    assert_eq!(cart_item.cart_id, 1);
    assert_eq!(cart_item.product_id, 1);
}

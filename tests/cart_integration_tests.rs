use cto_parallel_test::auth::{create_token, validate_token};
use cto_parallel_test::cart::CartService;
use cto_parallel_test::catalog::{NewProduct, ProductService};
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;

/// Helper function to simulate JWT extraction from Authorization header
fn extract_user_id_from_token(token: &str) -> Result<i32, String> {
    validate_token(token)
        .map(|claims| claims.sub.parse::<i32>().unwrap_or(0))
        .map_err(|_| "Invalid token".to_string())
}

#[test]
fn test_complete_shopping_cart_flow() {
    // Setup services
    let product_service = ProductService::new();
    let cart_service = CartService::new();

    // Create products
    let product1 = product_service.create(NewProduct {
        name: "Laptop".to_string(),
        description: "High-end laptop".to_string(),
        price: Decimal::from_f64(999.99).unwrap(),
        inventory_count: 10,
    });

    let product2 = product_service.create(NewProduct {
        name: "Mouse".to_string(),
        description: "Wireless mouse".to_string(),
        price: Decimal::from_f64(29.99).unwrap(),
        inventory_count: 50,
    });

    // Create JWT for user
    let user_id = 1;
    let token = create_token(user_id).expect("Token creation failed");

    // Validate token (simulating middleware)
    let extracted_user_id = extract_user_id_from_token(&token).expect("Token validation failed");
    assert_eq!(extracted_user_id, user_id);

    // User adds items to cart
    let cart = cart_service.add_item(user_id, &product1, 2);
    assert_eq!(cart.items.len(), 1);
    assert_eq!(cart.items[0].quantity, 2);

    let cart = cart_service.add_item(user_id, &product2, 5);
    assert_eq!(cart.items.len(), 2);

    // Get cart
    let retrieved_cart = cart_service.get_cart(user_id);
    assert!(retrieved_cart.is_some());
    let retrieved_cart = retrieved_cart.unwrap();
    assert_eq!(retrieved_cart.items.len(), 2);
    assert_eq!(retrieved_cart.user_id, user_id);

    // Calculate total
    let expected_total = (Decimal::from_f64(999.99).unwrap() * Decimal::from(2))
        + (Decimal::from_f64(29.99).unwrap() * Decimal::from(5));
    assert_eq!(retrieved_cart.total(), expected_total);

    // Remove one item
    let cart = cart_service.remove_item(user_id, product1.id);
    assert!(cart.is_some());
    assert_eq!(cart.unwrap().items.len(), 1);

    // Clear cart
    let cart = cart_service.clear_cart(user_id);
    assert!(cart.is_some());
    assert_eq!(cart.unwrap().items.len(), 0);
}

#[test]
fn test_cart_isolation_between_users() {
    let product_service = ProductService::new();
    let cart_service = CartService::new();

    let product = product_service.create(NewProduct {
        name: "Keyboard".to_string(),
        description: "Mechanical keyboard".to_string(),
        price: Decimal::from_f64(149.99).unwrap(),
        inventory_count: 20,
    });

    // User 1 adds to cart
    let token1 = create_token(1).unwrap();
    let user1_id = extract_user_id_from_token(&token1).unwrap();
    let _ = cart_service.add_item(user1_id, &product, 2);

    // User 2 adds to cart
    let token2 = create_token(2).unwrap();
    let user2_id = extract_user_id_from_token(&token2).unwrap();
    let _ = cart_service.add_item(user2_id, &product, 3);

    // Verify isolation
    let cart1 = cart_service.get_cart(user1_id).unwrap();
    let cart2 = cart_service.get_cart(user2_id).unwrap();

    assert_eq!(cart1.items[0].quantity, 2);
    assert_eq!(cart2.items[0].quantity, 3);
    assert_ne!(cart1.id, cart2.id);
}

#[test]
fn test_inventory_validation_workflow() {
    let product_service = ProductService::new();
    let cart_service = CartService::new();

    // Create product with limited inventory
    let product = product_service.create(NewProduct {
        name: "Limited Edition Item".to_string(),
        description: "Only 5 available".to_string(),
        price: Decimal::from_f64(199.99).unwrap(),
        inventory_count: 5,
    });

    let user_id = 1;
    let token = create_token(user_id).unwrap();
    let extracted_user_id = extract_user_id_from_token(&token).unwrap();

    // Valid add (within inventory)
    let quantity_requested = 3;
    if product.inventory_count >= quantity_requested {
        let cart = cart_service.add_item(extracted_user_id, &product, quantity_requested);
        assert_eq!(cart.items[0].quantity, quantity_requested);
    } else {
        panic!("Should not reach here - inventory should be sufficient");
    }

    // Simulate checking for over-inventory request
    let excessive_quantity = 10;
    let validation_result = product.inventory_count >= excessive_quantity;
    assert!(!validation_result, "Should fail inventory validation");
}

#[test]
fn test_unauthorized_access_simulation() {
    // Simulate invalid token scenario
    let invalid_token = "invalid.token.string";
    let result = extract_user_id_from_token(invalid_token);
    assert!(result.is_err(), "Should reject invalid token");

    // Simulate missing token scenario
    let empty_token = "";
    let result = extract_user_id_from_token(empty_token);
    assert!(result.is_err(), "Should reject empty token");
}

#[test]
fn test_nonexistent_product_handling() {
    let product_service = ProductService::new();

    let user_id = 1;
    let token = create_token(user_id).unwrap();
    let _extracted_user_id = extract_user_id_from_token(&token).unwrap();

    // Try to add non-existent product
    let nonexistent_product_id = 9999;
    let result = product_service.get_by_id(nonexistent_product_id);

    // This should return None, simulating a 404 in an API
    assert!(result.is_none(), "Non-existent product should return None");

    // In an API, this would return 404 before reaching cart service
    // We verify the product lookup fails appropriately
}

#[test]
fn test_concurrent_cart_operations_with_auth() {
    use std::thread;

    let product_service = ProductService::new();
    let cart_service = CartService::new();

    // Create products
    for i in 0..10 {
        let _ = product_service.create(NewProduct {
            name: format!("Product {i}"),
            description: format!("Description {i}"),
            price: Decimal::from_f64(10.0 * f64::from(i + 1)).unwrap(),
            inventory_count: 100,
        });
    }

    let cart_service_clone1 = cart_service.clone();
    let cart_service_clone2 = cart_service.clone();
    let product_service_clone1 = product_service.clone();
    let product_service_clone2 = product_service.clone();

    // Simulate multiple users accessing carts concurrently
    let handle1 = thread::spawn(move || {
        for i in 1..=5 {
            let token = create_token(i).unwrap();
            let user_id = extract_user_id_from_token(&token).unwrap();
            let product = product_service_clone1.get_by_id(i).unwrap();
            let _ = cart_service_clone1.add_item(user_id, &product, 1);
        }
    });

    let handle2 = thread::spawn(move || {
        for i in 6..=10 {
            let token = create_token(i).unwrap();
            let user_id = extract_user_id_from_token(&token).unwrap();
            let product = product_service_clone2.get_by_id(i).unwrap();
            let _ = cart_service_clone2.add_item(user_id, &product, 2);
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    // Verify all carts were created correctly
    for user_id in 1..=10 {
        let cart = cart_service.get_cart(user_id);
        assert!(cart.is_some(), "Cart should exist for user {user_id}");
    }
}

#[test]
fn test_add_same_product_multiple_times() {
    let product_service = ProductService::new();
    let cart_service = CartService::new();

    let product = product_service.create(NewProduct {
        name: "Headphones".to_string(),
        description: "Noise-cancelling headphones".to_string(),
        price: Decimal::from_f64(299.99).unwrap(),
        inventory_count: 30,
    });

    let user_id = 1;
    let token = create_token(user_id).unwrap();
    let extracted_user_id = extract_user_id_from_token(&token).unwrap();

    // Add product multiple times
    let _ = cart_service.add_item(extracted_user_id, &product, 2);
    let _ = cart_service.add_item(extracted_user_id, &product, 3);
    let cart = cart_service.add_item(extracted_user_id, &product, 1);

    // Should have one item with combined quantity
    assert_eq!(cart.items.len(), 1);
    assert_eq!(cart.items[0].quantity, 6);
}

#[test]
fn test_cart_total_calculation_accuracy() {
    let product_service = ProductService::new();
    let cart_service = CartService::new();

    let product1 = product_service.create(NewProduct {
        name: "Item A".to_string(),
        description: "Test item A".to_string(),
        price: Decimal::from_f64(19.99).unwrap(),
        inventory_count: 100,
    });

    let product2 = product_service.create(NewProduct {
        name: "Item B".to_string(),
        description: "Test item B".to_string(),
        price: Decimal::from_f64(49.95).unwrap(),
        inventory_count: 100,
    });

    let user_id = 1;
    let token = create_token(user_id).unwrap();
    let extracted_user_id = extract_user_id_from_token(&token).unwrap();

    let _ = cart_service.add_item(extracted_user_id, &product1, 3); // 59.97
    let cart = cart_service.add_item(extracted_user_id, &product2, 2); // 99.90

    let expected_total = Decimal::from_f64(159.87).unwrap();
    assert_eq!(cart.total(), expected_total);
}

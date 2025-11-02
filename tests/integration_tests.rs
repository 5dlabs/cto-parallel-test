// End-to-end integration tests
// Tests complete user flows combining authentication and product catalog

mod common;

use ecommerce_catalog::auth::{create_token, validate_token};
use ecommerce_catalog::catalog::{NewProduct, ProductFilter, ProductService};
use rust_decimal_macros::dec;

#[test]
fn test_complete_shopping_flow() {
    // 1. Initialize product catalog
    let product_service = common::create_test_product_service();

    // 2. User browses products
    let all_products = product_service.get_all();
    assert_eq!(all_products.len(), 5);

    // 3. User searches for specific products
    let filter = ProductFilter::new()
        .with_name("Mouse".to_string())
        .with_in_stock(true);
    let search_results = product_service.filter(&filter);
    assert_eq!(search_results.len(), 1);
    let selected_product = &search_results[0];

    // 4. User views product details
    let product_details = product_service.get_by_id(selected_product.id);
    assert!(product_details.is_some());
    let product = product_details.unwrap();
    assert_eq!(product.name, "Wireless Mouse");
    assert_eq!(product.price, dec!(29.99));
    assert!(product.inventory_count > 0);

    // 5. User "purchases" product (inventory reduced)
    let new_inventory = product.inventory_count - 1;
    let updated = product_service
        .update_inventory(product.id, new_inventory)
        .expect("Should update inventory");
    assert_eq!(updated.inventory_count, new_inventory);
}

#[test]
fn test_user_registration_and_login_flow() {
    // 1. User registers with credentials
    let password = "secure_password_123";
    let user = common::create_test_user(1, "newuser", "newuser@example.com", password);

    // 2. Verify user data is properly stored
    assert_eq!(user.username, "newuser");
    assert_eq!(user.email, "newuser@example.com");
    assert!(!user.password_hash.is_empty());

    // 3. User attempts login with correct password
    assert!(
        user.verify_password(password),
        "Login should succeed with correct password"
    );

    // 4. Generate JWT token for authenticated user
    let token = create_token(&user.id.to_string()).expect("Failed to create token");
    assert!(!token.is_empty());

    // 5. Validate token for subsequent requests
    let claims = validate_token(&token).expect("Token should be valid");
    assert_eq!(claims.sub, user.id.to_string());

    // 6. Verify wrong password fails
    assert!(
        !user.verify_password("wrong_password"),
        "Login should fail with wrong password"
    );
}

#[test]
fn test_authenticated_user_shopping_flow() {
    // 1. User authenticates
    let user = common::create_test_user(1, "shopper", "shopper@example.com", "password123");
    assert!(user.verify_password("password123"));

    let token = create_token(&user.id.to_string()).expect("Failed to create token");
    let claims = validate_token(&token).expect("Token should be valid");
    assert_eq!(claims.sub, "1");

    // 2. Authenticated user browses products
    let product_service = common::create_test_product_service();
    let products = product_service.get_all();
    assert!(!products.is_empty());

    // 3. User adds multiple items to cart (simulated by inventory check)
    let laptop = product_service.get_by_id(1).expect("Laptop should exist");
    let mouse = product_service.get_by_id(2).expect("Mouse should exist");

    assert!(laptop.inventory_count > 0, "Laptop should be in stock");
    assert!(mouse.inventory_count > 0, "Mouse should be in stock");

    // 4. User proceeds with purchase (inventory reduced)
    let _ = product_service.update_inventory(laptop.id, laptop.inventory_count - 1);
    let _ = product_service.update_inventory(mouse.id, mouse.inventory_count - 1);

    // 5. Verify inventory was updated
    let updated_laptop = product_service.get_by_id(laptop.id).unwrap();
    let updated_mouse = product_service.get_by_id(mouse.id).unwrap();

    assert_eq!(updated_laptop.inventory_count, laptop.inventory_count - 1);
    assert_eq!(updated_mouse.inventory_count, mouse.inventory_count - 1);
}

#[test]
fn test_multiple_users_independent_sessions() {
    // Create multiple users
    let user1 = common::create_test_user(1, "user1", "user1@example.com", "pass1");
    let user2 = common::create_test_user(2, "user2", "user2@example.com", "pass2");
    let user3 = common::create_test_user(3, "user3", "user3@example.com", "pass3");

    // Each user authenticates independently
    let token1 = create_token(&user1.id.to_string()).expect("Failed to create token 1");
    let token2 = create_token(&user2.id.to_string()).expect("Failed to create token 2");
    let token3 = create_token(&user3.id.to_string()).expect("Failed to create token 3");

    // Validate all tokens
    let claims1 = validate_token(&token1).expect("Token 1 should be valid");
    let claims2 = validate_token(&token2).expect("Token 2 should be valid");
    let claims3 = validate_token(&token3).expect("Token 3 should be valid");

    assert_eq!(claims1.sub, "1");
    assert_eq!(claims2.sub, "2");
    assert_eq!(claims3.sub, "3");

    // All users share the same product catalog
    let product_service = common::create_test_product_service();

    let user1_view = product_service.get_all();
    let user2_view = product_service.get_all();
    let user3_view = product_service.get_all();

    assert_eq!(user1_view.len(), user2_view.len());
    assert_eq!(user2_view.len(), user3_view.len());
}

#[test]
fn test_product_search_and_filter_flow() {
    let product_service = common::create_test_product_service();

    // User searches by name
    let name_filter = ProductFilter::new().with_name("laptop".to_string());
    let name_results = product_service.filter(&name_filter);
    assert_eq!(name_results.len(), 1);
    assert_eq!(name_results[0].name, "Laptop");

    // User filters by price range
    let price_filter = ProductFilter::new()
        .with_min_price(dec!(25.00))
        .with_max_price(dec!(50.00));
    let price_results = product_service.filter(&price_filter);
    assert_eq!(price_results.len(), 2); // Mouse (29.99) and USB-C Hub (45.99)

    // User filters for in-stock items
    let stock_filter = ProductFilter::new().with_in_stock(true);
    let stock_results = product_service.filter(&stock_filter);
    assert_eq!(stock_results.len(), 4); // All except 4K Monitor

    // User combines multiple filters
    let combined_filter = ProductFilter::new()
        .with_in_stock(true)
        .with_max_price(dec!(100.00));
    let combined_results = product_service.filter(&combined_filter);
    assert!(!combined_results.is_empty());
    for product in combined_results {
        assert!(product.inventory_count > 0);
        assert!(product.price <= dec!(100.00));
    }
}

#[test]
fn test_out_of_stock_handling() {
    let product_service = common::create_test_product_service();

    // Find out-of-stock product (4K Monitor)
    let monitor = product_service.get_by_id(4).expect("Monitor should exist");
    assert_eq!(monitor.inventory_count, 0);

    // Filter should correctly identify out-of-stock items
    let out_of_stock_filter = ProductFilter::new().with_in_stock(false);
    let out_of_stock = product_service.filter(&out_of_stock_filter);
    assert_eq!(out_of_stock.len(), 1);
    assert_eq!(out_of_stock[0].name, "4K Monitor");

    // In-stock filter should exclude it
    let in_stock_filter = ProductFilter::new().with_in_stock(true);
    let in_stock = product_service.filter(&in_stock_filter);
    assert!(in_stock.iter().all(|p| p.id != 4));
}

#[test]
fn test_inventory_management_flow() {
    let product_service = ProductService::new();

    // 1. Add new product with initial inventory
    let product = product_service.create(NewProduct {
        name: "New Product".to_string(),
        description: "Just added to catalog".to_string(),
        price: dec!(100.00),
        inventory_count: 50,
    });

    // 2. Verify initial state
    assert_eq!(product.inventory_count, 50);

    // 3. Simulate sales (reduce inventory)
    let _ = product_service.update_inventory(product.id, 45);
    let _ = product_service.update_inventory(product.id, 40);
    let _ = product_service.update_inventory(product.id, 35);

    let current_state = product_service.get_by_id(product.id).unwrap();
    assert_eq!(current_state.inventory_count, 35);

    // 4. Restock (increase inventory)
    let _ = product_service.update_inventory(product.id, 100);
    let restocked = product_service.get_by_id(product.id).unwrap();
    assert_eq!(restocked.inventory_count, 100);

    // 5. Product goes out of stock
    let _ = product_service.update_inventory(product.id, 0);
    let out_of_stock = product_service.get_by_id(product.id).unwrap();
    assert_eq!(out_of_stock.inventory_count, 0);

    // 6. Verify out-of-stock filtering works
    let filter = ProductFilter::new().with_in_stock(false);
    let results = product_service.filter(&filter);
    assert!(results.iter().any(|p| p.id == product.id));
}

#[test]
fn test_product_lifecycle() {
    let product_service = ProductService::new();

    // 1. Create product
    let product = product_service.create(NewProduct {
        name: "Lifecycle Product".to_string(),
        description: "Testing full lifecycle".to_string(),
        price: dec!(50.00),
        inventory_count: 10,
    });
    let product_id = product.id;

    // 2. Retrieve product
    let retrieved = product_service.get_by_id(product_id);
    assert!(retrieved.is_some());

    // 3. Update inventory multiple times
    let _ = product_service.update_inventory(product_id, 20);
    let _ = product_service.update_inventory(product_id, 15);
    let _ = product_service.update_inventory(product_id, 5);

    // 4. Verify updates
    let updated = product_service.get_by_id(product_id).unwrap();
    assert_eq!(updated.inventory_count, 5);

    // 5. Delete product
    let deleted = product_service.delete(product_id);
    assert!(deleted);

    // 6. Verify deletion
    let after_delete = product_service.get_by_id(product_id);
    assert!(after_delete.is_none());
}

#[test]
fn test_concurrent_user_operations() {
    use std::sync::Arc;
    use std::thread;

    let product_service = Arc::new(common::create_test_product_service());

    // Multiple users accessing the catalog simultaneously
    let mut handles = vec![];

    for i in 0..5 {
        let service_clone = Arc::clone(&product_service);
        let handle = thread::spawn(move || {
            // Each user browses products
            let products = service_clone.get_all();
            assert_eq!(products.len(), 5);

            // Each user searches for products
            let filter = ProductFilter::new().with_in_stock(true);
            let results = service_clone.filter(&filter);
            assert!(!results.is_empty());

            // Each user views a specific product
            let product = service_clone.get_by_id(i % 5 + 1);
            assert!(product.is_some());
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Thread panicked");
    }
}

#[test]
fn test_error_handling_invalid_product_id() {
    let product_service = common::create_test_product_service();

    // Test various invalid IDs
    let invalid_ids = vec![0, -1, -100, 999, 10000];

    for id in invalid_ids {
        // Get should return None
        let result = product_service.get_by_id(id);
        assert!(result.is_none(), "Invalid ID {id} should return None");

        // Update should return None
        let update_result = product_service.update_inventory(id, 10);
        assert!(
            update_result.is_none(),
            "Update with invalid ID {id} should return None"
        );

        // Delete should return false
        let delete_result = product_service.delete(id);
        assert!(
            !delete_result,
            "Delete with invalid ID {id} should return false"
        );
    }
}

#[test]
fn test_filter_edge_cases() {
    let product_service = common::create_test_product_service();

    // Empty name filter
    let empty_name_filter = ProductFilter::new().with_name(String::new());
    let results = product_service.filter(&empty_name_filter);
    assert_eq!(results.len(), 5); // Empty string matches all

    // Very high min price (no matches)
    let high_price_filter = ProductFilter::new().with_min_price(dec!(10000.00));
    let results = product_service.filter(&high_price_filter);
    assert_eq!(results.len(), 0);

    // Very low max price (no matches)
    let low_price_filter = ProductFilter::new().with_max_price(dec!(0.01));
    let results = product_service.filter(&low_price_filter);
    assert_eq!(results.len(), 0);

    // Inverted price range (min > max, should return nothing)
    let inverted_filter = ProductFilter::new()
        .with_min_price(dec!(1000.00))
        .with_max_price(dec!(100.00));
    let results = product_service.filter(&inverted_filter);
    assert_eq!(results.len(), 0);
}

#[test]
fn test_authentication_error_handling() {
    let user = common::create_test_user(1, "testuser", "test@example.com", "correct_pass");

    // Test various wrong passwords
    let wrong_passwords = vec![
        "wrong_pass",
        "correct_pass1", // Extra character
        "correct_pas",   // Missing character
        "Correct_pass",  // Wrong case
        "",              // Empty
        "completely_different",
    ];

    for wrong_pass in wrong_passwords {
        assert!(
            !user.verify_password(wrong_pass),
            "Wrong password should fail: {wrong_pass}"
        );
    }

    // Test invalid tokens
    let invalid_tokens = vec![
        "not.a.token",
        "invalid_token",
        "",
        "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.invalid.signature",
    ];

    for invalid_token in invalid_tokens {
        let result = validate_token(invalid_token);
        assert!(
            result.is_err(),
            "Invalid token should be rejected: {invalid_token}"
        );
    }
}

#[test]
fn test_complete_e2e_scenario() {
    // Simulates a complete e-commerce flow from registration to purchase

    // 1. Setup product catalog
    let product_service = common::create_test_product_service();

    // 2. User registers
    let user = common::create_test_user(1, "john_doe", "john@example.com", "MyP@ssw0rd!");

    // 3. User logs in
    assert!(user.verify_password("MyP@ssw0rd!"));
    let token = create_token(&user.id.to_string()).expect("Failed to create token");

    // 4. Validate user session
    let claims = validate_token(&token).expect("Token should be valid");
    assert_eq!(claims.sub, "1");

    // 5. User browses products
    let all_products = product_service.get_all();
    assert_eq!(all_products.len(), 5);

    // 6. User searches for affordable items
    let affordable_filter = ProductFilter::new()
        .with_max_price(dec!(100.00))
        .with_in_stock(true);
    let affordable_products = product_service.filter(&affordable_filter);
    assert!(!affordable_products.is_empty());

    // 7. User selects products
    let mouse = product_service.get_by_id(2).expect("Mouse should exist");
    let hub = product_service.get_by_id(5).expect("Hub should exist");

    assert_eq!(mouse.name, "Wireless Mouse");
    assert_eq!(hub.name, "USB-C Hub");

    // 8. User checks total price
    let total = mouse.price + hub.price;
    assert_eq!(total, dec!(75.98)); // 29.99 + 45.99

    // 9. User completes purchase (inventory updated)
    let initial_mouse_inventory = mouse.inventory_count;
    let initial_hub_inventory = hub.inventory_count;

    let _ = product_service.update_inventory(mouse.id, initial_mouse_inventory - 1);
    let _ = product_service.update_inventory(hub.id, initial_hub_inventory - 1);

    // 10. Verify final state
    let final_mouse = product_service.get_by_id(mouse.id).unwrap();
    let final_hub = product_service.get_by_id(hub.id).unwrap();

    assert_eq!(final_mouse.inventory_count, initial_mouse_inventory - 1);
    assert_eq!(final_hub.inventory_count, initial_hub_inventory - 1);

    // 11. User session remains valid
    let final_claims = validate_token(&token).expect("Token should still be valid");
    assert_eq!(final_claims.sub, "1");
}

#[test]
fn test_product_service_isolation() {
    // Verify that different service instances are independent
    let service1 = ProductService::new();
    let service2 = ProductService::new();

    // Add product to service1
    let product1 = service1.create(NewProduct {
        name: "Service 1 Product".to_string(),
        description: "Only in service 1".to_string(),
        price: dec!(100.00),
        inventory_count: 10,
    });

    // Should not exist in service2
    let result = service2.get_by_id(product1.id);
    assert!(
        result.is_none(),
        "Product should not exist in different service instance"
    );

    // service2 should be empty
    assert_eq!(service2.get_all().len(), 0);
}

#[test]
fn test_user_data_serialization_security() {
    // Verify that sensitive data is not exposed in serialization
    let user = common::create_test_user(1, "secureuser", "secure@example.com", "super_secret");

    let json = serde_json::to_string(&user).expect("Failed to serialize user");

    // Password hash should not be in JSON
    assert!(!json.contains("password_hash"));
    assert!(!json.contains("super_secret"));

    // Verify the actual hash value is not exposed
    assert!(!json.contains(&user.password_hash));

    // Public fields should be present
    assert!(json.contains("secureuser"));
    assert!(json.contains("secure@example.com"));
}

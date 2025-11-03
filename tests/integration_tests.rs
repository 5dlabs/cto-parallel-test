//! End-to-end integration tests
//!
//! Tests complete user flows including authentication, product browsing, and cart operations.

mod common;

use cto_parallel_test::auth::jwt::{create_token, validate_token};
use cto_parallel_test::catalog::models::ProductFilter;
use cto_parallel_test::catalog::service::ProductService;

// Complete User Shopping Flow

#[test]
fn test_complete_shopping_flow() {
    // Step 1: User browses products
    let product_service = common::create_test_product_service();
    let products = product_service.get_all();

    assert_eq!(products.len(), 3);
    assert!(products.iter().any(|p| p.name == "Laptop"));

    // Step 2: User views specific product details
    let laptop = product_service.get_by_id(1).expect("Laptop should exist");
    assert_eq!(laptop.name, "Laptop");
    assert_eq!(laptop.inventory_count, 10);

    // Step 3: User registers/authenticates
    let user = common::create_test_user(1, "shopper", "password123");
    assert!(user.verify_password("password123"));

    // Step 4: User gets JWT token after login
    let token = create_token(&user.id.to_string()).expect("Failed to create token");
    let claims = validate_token(&token).expect("Failed to validate token");
    assert_eq!(claims.sub, "1");

    // Step 5: User adds items to cart (simulated by checking inventory)
    assert!(laptop.inventory_count > 0, "Product should be in stock");

    // Step 6: User completes checkout (verified by updating inventory)
    let updated = product_service.update_inventory(laptop.id, laptop.inventory_count - 1);
    assert!(updated.is_some());
    assert_eq!(updated.unwrap().inventory_count, 9);
}

#[test]
fn test_user_registration_and_login_flow() {
    // Step 1: User registers with credentials
    let password = "secure_password_123";
    let user = common::create_test_user(1, "new_user", password);

    assert_eq!(user.username, "new_user");
    assert_eq!(user.email, "new_user@example.com");

    // Step 2: Password is hashed securely
    assert_ne!(user.password_hash, password);
    assert!(user.password_hash.starts_with("$argon2"));

    // Step 3: User logs in with correct credentials
    assert!(
        user.verify_password(password),
        "Should authenticate with correct password"
    );

    // Step 4: JWT token is generated
    let token = create_token(&user.id.to_string()).expect("Failed to create token");
    assert!(!token.is_empty());

    // Step 5: Token is validated on subsequent requests
    let claims = validate_token(&token).expect("Failed to validate token");
    assert_eq!(claims.sub, user.id.to_string());

    // Step 6: Invalid password is rejected
    assert!(!user.verify_password("wrong_password"));
}

#[test]
fn test_product_search_and_filter_flow() {
    // Setup: Create diverse product catalog
    let service = ProductService::new();
    let _ = service.create(common::create_test_product("Gaming Laptop", 1299.99, 5));
    let _ = service.create(common::create_test_product("Office Laptop", 799.99, 10));
    let _ = service.create(common::create_test_product("Gaming Mouse", 59.99, 25));
    let _ = service.create(common::create_test_product(
        "Mechanical Keyboard",
        129.99,
        0,
    ));
    let _ = service.create(common::create_test_product("USB Mouse", 19.99, 50));

    // Flow 1: User searches for "laptop"
    let laptop_filter = ProductFilter {
        name_contains: Some("laptop".to_string()),
        min_price: None,
        max_price: None,
        in_stock: None,
    };
    let laptop_results = service.filter(&laptop_filter);
    assert_eq!(laptop_results.len(), 2);

    // Flow 2: User filters by price range
    let price_filter = ProductFilter {
        name_contains: None,
        min_price: Some(common::decimal_from_str("50.0")),
        max_price: Some(common::decimal_from_str("150.0")),
        in_stock: None,
    };
    let price_results = service.filter(&price_filter);
    assert_eq!(price_results.len(), 2); // Gaming Mouse (59.99) and Keyboard (129.99)

    // Flow 3: User filters for in-stock items only
    let stock_filter = ProductFilter {
        name_contains: None,
        min_price: None,
        max_price: None,
        in_stock: Some(true),
    };
    let stock_results = service.filter(&stock_filter);
    assert_eq!(stock_results.len(), 4); // All except Keyboard (0 inventory)

    // Flow 4: User combines multiple filters
    let combined_filter = ProductFilter {
        name_contains: Some("gaming".to_string()),
        min_price: Some(common::decimal_from_str("50.0")),
        max_price: Some(common::decimal_from_str("100.0")),
        in_stock: Some(true),
    };
    let combined_results = service.filter(&combined_filter);
    assert_eq!(combined_results.len(), 1);
    assert_eq!(combined_results[0].name, "Gaming Mouse");
}

#[test]
fn test_inventory_management_flow() {
    let service = ProductService::new();

    // Step 1: Add new products to catalog
    let product = service.create(common::create_test_product("New Product", 49.99, 100));
    assert_eq!(product.inventory_count, 100);

    // Step 2: Customer purchases items (reduce inventory)
    let after_sale1 = service.update_inventory(product.id, 95);
    assert_eq!(after_sale1.unwrap().inventory_count, 95);

    // Step 3: Another customer purchases
    let after_sale2 = service.update_inventory(product.id, 90);
    assert_eq!(after_sale2.unwrap().inventory_count, 90);

    // Step 4: Restock inventory
    let after_restock = service.update_inventory(product.id, 150);
    assert_eq!(after_restock.unwrap().inventory_count, 150);

    // Step 5: Product goes out of stock
    let out_of_stock = service.update_inventory(product.id, 0);
    assert_eq!(out_of_stock.unwrap().inventory_count, 0);

    // Step 6: Check product is marked as out of stock
    let filter = ProductFilter {
        name_contains: None,
        min_price: None,
        max_price: None,
        in_stock: Some(false),
    };
    let out_of_stock_products = service.filter(&filter);
    assert_eq!(out_of_stock_products.len(), 1);
}

#[test]
fn test_multi_user_authentication_flow() {
    // Multiple users register
    let user1 = common::create_test_user(1, "alice", "alice_pass");
    let user2 = common::create_test_user(2, "bob", "bob_pass");
    let user3 = common::create_test_user(3, "charlie", "charlie_pass");

    // Each user can authenticate with their own credentials
    assert!(user1.verify_password("alice_pass"));
    assert!(user2.verify_password("bob_pass"));
    assert!(user3.verify_password("charlie_pass"));

    // Each user has unique token
    let token1 = create_token("1").unwrap();
    let token2 = create_token("2").unwrap();
    let token3 = create_token("3").unwrap();

    assert_ne!(token1, token2);
    assert_ne!(token2, token3);
    assert_ne!(token1, token3);

    // Tokens identify correct users
    assert_eq!(validate_token(&token1).unwrap().sub, "1");
    assert_eq!(validate_token(&token2).unwrap().sub, "2");
    assert_eq!(validate_token(&token3).unwrap().sub, "3");

    // Users cannot authenticate with wrong passwords
    assert!(!user1.verify_password("bob_pass"));
    assert!(!user2.verify_password("charlie_pass"));
    assert!(!user3.verify_password("alice_pass"));
}

#[test]
fn test_product_lifecycle_flow() {
    let service = ProductService::new();

    // Step 1: Product is created
    let product = service.create(common::create_test_product("Lifecycle Product", 99.99, 50));
    let product_id = product.id;

    // Step 2: Product is retrievable
    assert!(service.get_by_id(product_id).is_some());

    // Step 3: Product details are updated (inventory)
    let updated = service.update_inventory(product_id, 25);
    assert_eq!(updated.unwrap().inventory_count, 25);

    // Step 4: Product is still in catalog
    let all_products = service.get_all();
    assert_eq!(all_products.len(), 1);

    // Step 5: Product is deleted
    assert!(service.delete(product_id));

    // Step 6: Product is no longer retrievable
    assert!(service.get_by_id(product_id).is_none());

    // Step 7: Catalog is empty
    assert!(service.get_all().is_empty());
}

#[test]
fn test_concurrent_shopping_flow() {
    use std::thread;

    let service = ProductService::new();

    // Setup: Create products
    let p1 = service.create(common::create_test_product("Product 1", 10.0, 100));
    let p2 = service.create(common::create_test_product("Product 2", 20.0, 100));

    let service_clone1 = service.clone();
    let service_clone2 = service.clone();

    // Simulate two users shopping concurrently
    let user1_thread = thread::spawn(move || {
        // User 1 browses and "purchases" items
        let products = service_clone1.get_all();
        assert_eq!(products.len(), 2);

        // User 1 reduces inventory
        let _ = service_clone1.update_inventory(1, 95);
    });

    let user2_thread = thread::spawn(move || {
        // User 2 browses and "purchases" items
        let products = service_clone2.get_all();
        assert_eq!(products.len(), 2);

        // User 2 reduces inventory
        let _ = service_clone2.update_inventory(2, 95);
    });

    user1_thread.join().unwrap();
    user2_thread.join().unwrap();

    // Verify both updates succeeded
    let updated_p1 = service.get_by_id(p1.id).unwrap();
    let updated_p2 = service.get_by_id(p2.id).unwrap();

    assert_eq!(updated_p1.inventory_count, 95);
    assert_eq!(updated_p2.inventory_count, 95);
}

#[test]
fn test_authentication_and_product_access_integration() {
    // User authenticates
    let user = common::create_test_user(1, "authenticated_user", "secure123");
    let token = create_token(&user.id.to_string()).expect("Failed to create token");

    // Validate token before allowing product access
    let claims = validate_token(&token).expect("Token should be valid");
    assert_eq!(claims.sub, "1");

    // Now user can access products
    let service = common::create_test_product_service();
    let products = service.get_all();
    assert_eq!(products.len(), 3);

    // User can view product details
    let product = service.get_by_id(1).expect("Product should exist");
    assert!(product.inventory_count > 0);
}

#[test]
fn test_invalid_authentication_blocks_access() {
    // User attempts to access with invalid token
    let invalid_token = "invalid.jwt.token";
    let result = validate_token(invalid_token);

    // Token validation should fail
    assert!(result.is_err(), "Invalid token should not be accepted");

    // This would prevent access to protected resources in a real API
}

#[test]
fn test_product_search_with_no_results() {
    let service = common::create_test_product_service();

    // User searches for non-existent product
    let filter = ProductFilter {
        name_contains: Some("NonExistent".to_string()),
        min_price: None,
        max_price: None,
        in_stock: None,
    };

    let results = service.filter(&filter);
    assert!(results.is_empty(), "Should return empty results");
}

#[test]
fn test_out_of_stock_product_handling() {
    let service = ProductService::new();

    // Create out of stock product
    let product = service.create(common::create_test_product("Sold Out Item", 99.99, 0));

    // User searches for in-stock items only
    let filter = ProductFilter {
        name_contains: None,
        min_price: None,
        max_price: None,
        in_stock: Some(true),
    };

    let results = service.filter(&filter);
    assert!(
        results.is_empty(),
        "Out of stock product should not appear in in-stock filter"
    );

    // User can still view the product details
    let product_details = service.get_by_id(product.id).expect("Product should exist");
    assert_eq!(product_details.inventory_count, 0);
}

#[test]
fn test_price_comparison_shopping_flow() {
    let service = ProductService::new();

    // User browses products in different price ranges
    let _ = service.create(common::create_test_product("Budget Option", 19.99, 10));
    let _ = service.create(common::create_test_product("Mid-Range Option", 49.99, 10));
    let _ = service.create(common::create_test_product("Premium Option", 99.99, 10));

    // User looks for budget options (under $30)
    let budget_filter = ProductFilter {
        name_contains: None,
        min_price: None,
        max_price: Some(common::decimal_from_str("30.0")),
        in_stock: None,
    };
    let budget_results = service.filter(&budget_filter);
    assert_eq!(budget_results.len(), 1);
    assert_eq!(budget_results[0].name, "Budget Option");

    // User looks for mid-range ($40-$60)
    let mid_filter = ProductFilter {
        name_contains: None,
        min_price: Some(common::decimal_from_str("40.0")),
        max_price: Some(common::decimal_from_str("60.0")),
        in_stock: None,
    };
    let mid_results = service.filter(&mid_filter);
    assert_eq!(mid_results.len(), 1);
    assert_eq!(mid_results[0].name, "Mid-Range Option");

    // User looks for premium ($80+)
    let premium_filter = ProductFilter {
        name_contains: None,
        min_price: Some(common::decimal_from_str("80.0")),
        max_price: None,
        in_stock: None,
    };
    let premium_results = service.filter(&premium_filter);
    assert_eq!(premium_results.len(), 1);
    assert_eq!(premium_results[0].name, "Premium Option");
}

#[test]
fn test_user_session_expiration() {
    // User logs in
    let user = common::create_test_user(1, "session_user", "pass123");
    let token = create_token(&user.id.to_string()).expect("Failed to create token");

    // Token is valid initially
    let claims = validate_token(&token).expect("Token should be valid");
    assert_eq!(claims.sub, "1");

    // Verify token has expiration time set (24 hours from now)
    let time_to_expiry = claims.exp - claims.iat;
    assert!(
        (86390..=86410).contains(&time_to_expiry),
        "Token should expire in approximately 24 hours"
    );
}

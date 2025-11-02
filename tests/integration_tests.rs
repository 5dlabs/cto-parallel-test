mod common;

use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use cto_parallel_test::catalog::{ProductFilter, ProductService};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::sync::Arc;
use std::thread;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct User {
    id: i32,
    email: String,
    password_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    iat: usize,
}

// Test fixture for JWT operations  
const JWT_TEST_KEY: &str = "test-jwt-secret";

/// Helper function to convert timestamp to usize
#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
fn timestamp_to_usize(timestamp: i64) -> usize {
    timestamp as usize
}

/// Helper function to create a JWT token for a user
fn create_user_token(user_email: &str) -> String {
    let now = Utc::now();
    let claims = Claims {
        sub: user_email.to_string(),
        iat: timestamp_to_usize(now.timestamp()),
        exp: timestamp_to_usize((now + Duration::hours(24)).timestamp()),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_TEST_KEY.as_ref()),
    )
    .expect("Failed to create JWT token")
}

/// Helper function to validate a JWT token
fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_TEST_KEY.as_ref()),
        &Validation::new(Algorithm::HS256),
    )?;
    Ok(token_data.claims)
}

#[test]
fn test_user_registration_and_login_flow() {
    // Step 1: User Registration
    let user_email = "test@example.com";
    let user_password = "secure_password123";

    let password_hash = hash(user_password, DEFAULT_COST).expect("Failed to hash password");

    let user = User {
        id: 1,
        email: user_email.to_string(),
        password_hash: password_hash.clone(),
    };

    assert_eq!(user.email, "test@example.com");
    assert_ne!(user.password_hash, user_password);

    // Step 2: User Login
    let login_password = "secure_password123";
    let is_valid = verify(login_password, &user.password_hash).expect("Failed to verify password");

    assert!(is_valid);

    // Step 3: Create JWT token upon successful login
    let token = create_user_token(&user.email);

    assert!(!token.is_empty());

    // Step 4: Validate token
    let claims = validate_token(&token).expect("Failed to validate token");

    assert_eq!(claims.sub, user.email);
}

#[test]
fn test_failed_login_flow() {
    // Step 1: User Registration
    let user_password = "correct_password";
    let password_hash = hash(user_password, DEFAULT_COST).expect("Failed to hash password");

    let user = User {
        id: 1,
        email: "user@example.com".to_string(),
        password_hash,
    };

    // Step 2: Failed login attempt with wrong password
    let wrong_password = "wrong_password";
    let is_valid = verify(wrong_password, &user.password_hash).expect("Failed to verify password");

    assert!(!is_valid);
    // No token should be issued for failed login
}

#[test]
fn test_shopping_flow_browse_add_to_cart() {
    // Step 1: Browse products
    let product_service = common::create_test_product_service();
    let all_products = product_service.get_all();

    assert_eq!(all_products.len(), 4);

    // Step 2: Filter products
    let mut filter = ProductFilter::new();
    filter.name_contains = Some("Apple".to_string());
    filter.in_stock = Some(true);

    let filtered_products = product_service.filter(&filter);

    assert!(!filtered_products.is_empty());

    // Step 3: Select a product
    let selected_product = filtered_products[0].clone();

    assert!(selected_product.inventory_count > 0);

    // Step 4: Simulate adding to cart (reduce inventory)
    let cart_quantity = 2;
    let new_inventory = selected_product.inventory_count - cart_quantity;

    let updated = product_service
        .update_inventory(selected_product.id, new_inventory)
        .expect("Failed to update inventory");

    assert_eq!(updated.inventory_count, new_inventory);

    // Step 5: Verify inventory was updated
    let product_after_cart = product_service
        .get_by_id(selected_product.id)
        .expect("Product not found");

    assert_eq!(product_after_cart.inventory_count, new_inventory);
}

#[test]
fn test_authenticated_shopping_flow() {
    // Step 1: User registration and login
    let user_email = "shopper@example.com";
    let user_password = "shopping_password";
    let password_hash = hash(user_password, DEFAULT_COST).expect("Failed to hash password");

    let user = User {
        id: 1,
        email: user_email.to_string(),
        password_hash,
    };

    // Authenticate user
    let is_authenticated = verify(user_password, &user.password_hash).unwrap();
    assert!(is_authenticated);

    let token = create_user_token(&user.email);

    // Step 2: Validate token before allowing shopping
    let claims = validate_token(&token).expect("Invalid token");
    assert_eq!(claims.sub, user.email);

    // Step 3: Browse products (authenticated)
    let product_service = common::create_test_product_service();
    let products = product_service.get_all();

    assert!(!products.is_empty());

    // Step 4: Add product to cart
    let product = products[0].clone();
    let cart_item_quantity = 1;

    // Verify inventory before purchase
    assert!(product.inventory_count > 0);

    // Simulate cart operation
    let new_inventory = product.inventory_count - cart_item_quantity;
    let updated = product_service
        .update_inventory(product.id, new_inventory)
        .expect("Failed to update inventory");

    assert_eq!(updated.inventory_count, new_inventory);
}

#[test]
fn test_checkout_flow() {
    // Step 1: User authentication
    let token = create_user_token("buyer@example.com");
    let claims = validate_token(&token).expect("Invalid token");
    assert_eq!(claims.sub, "buyer@example.com");

    // Step 2: Get cart items (simulate)
    let product_service = common::create_test_product_service();
    let product1 = product_service.get_by_id(1).expect("Product not found");
    let product2 = product_service.get_by_id(2).expect("Product not found");

    let cart_items = vec![(product1.clone(), 2), (product2.clone(), 1)];

    // Step 3: Calculate total
    let total: Decimal = cart_items
        .iter()
        .map(|(product, quantity)| product.price * Decimal::from(*quantity))
        .sum();

    assert!(total > Decimal::from(0));

    // Step 4: Process checkout (update inventory)
    for (product, quantity) in cart_items {
        let new_inventory = product.inventory_count - quantity;
        let updated = product_service.update_inventory(product.id, new_inventory);
        assert!(updated.is_some());
    }

    // Step 5: Verify inventory was reduced
    let product1_after = product_service.get_by_id(1).expect("Product not found");
    let product2_after = product_service.get_by_id(2).expect("Product not found");

    assert_eq!(product1_after.inventory_count, product1.inventory_count - 2);
    assert_eq!(product2_after.inventory_count, product2.inventory_count - 1);
}

#[test]
fn test_concurrent_shopping_operations() {
    let product_service = Arc::new(common::create_test_product_service());

    let service1 = Arc::clone(&product_service);
    let service2 = Arc::clone(&product_service);

    let handle1 = thread::spawn(move || {
        // User 1 browsing and filtering
        let mut filter = ProductFilter::new();
        filter.name_contains = Some("Apple".to_string());
        let products = service1.filter(&filter);
        assert!(!products.is_empty());
    });

    let handle2 = thread::spawn(move || {
        // User 2 browsing all products
        let products = service2.get_all();
        assert!(!products.is_empty());
    });

    handle1.join().expect("Thread 1 panicked");
    handle2.join().expect("Thread 2 panicked");
}

#[test]
fn test_out_of_stock_handling() {
    let product_service = common::create_test_product_service();

    // Find out of stock product
    let mut filter = ProductFilter::new();
    filter.in_stock = Some(false);
    let out_of_stock_products = product_service.filter(&filter);

    assert!(!out_of_stock_products.is_empty());

    let out_of_stock_product = &out_of_stock_products[0];
    assert_eq!(out_of_stock_product.inventory_count, 0);

    // Attempt to add to cart should be prevented in real implementation
    // Here we just verify the product is out of stock
    assert!(out_of_stock_product.inventory_count <= 0);
}

#[test]
fn test_price_calculation_accuracy() {
    let product_service = ProductService::new();

    let product =
        product_service.create(common::create_test_product("Expensive Item", "999.99", 10));

    let quantity = 3;
    let expected_total = Decimal::from_str("2999.97").unwrap();
    let calculated_total = product.price * Decimal::from(quantity);

    assert_eq!(calculated_total, expected_total);
}

#[test]
fn test_multiple_user_sessions() {
    // User 1 session
    let user1_token = create_user_token("user1@example.com");
    let user1_claims = validate_token(&user1_token).expect("Invalid token");
    assert_eq!(user1_claims.sub, "user1@example.com");

    // User 2 session
    let user2_token = create_user_token("user2@example.com");
    let user2_claims = validate_token(&user2_token).expect("Invalid token");
    assert_eq!(user2_claims.sub, "user2@example.com");

    // Tokens should be different
    assert_ne!(user1_token, user2_token);

    // Both users can browse products independently
    let product_service = common::create_test_product_service();

    let user1_products = product_service.get_all();
    let user2_products = product_service.get_all();

    assert_eq!(user1_products.len(), user2_products.len());
}

#[test]
fn test_search_and_filter_workflow() {
    let product_service = common::create_test_product_service();

    // Step 1: Search by keyword
    let mut search_filter = ProductFilter::new();
    search_filter.name_contains = Some("phone".to_string());
    let search_results = product_service.filter(&search_filter);

    assert!(!search_results.is_empty());

    // Step 2: Filter by price range
    let mut price_filter = ProductFilter::new();
    price_filter.min_price = Some(Decimal::from_str("500.00").unwrap());
    price_filter.max_price = Some(Decimal::from_str("1000.00").unwrap());
    let price_filtered = product_service.filter(&price_filter);

    assert!(!price_filtered.is_empty());

    // Step 3: Combine filters
    let mut combined_filter = ProductFilter::new();
    combined_filter.name_contains = Some("phone".to_string());
    combined_filter.min_price = Some(Decimal::from_str("500.00").unwrap());
    combined_filter.in_stock = Some(true);
    let final_results = product_service.filter(&combined_filter);

    // Results should match all criteria
    for product in final_results {
        assert!(product.name.to_lowercase().contains("phone"));
        assert!(product.price >= Decimal::from_str("500.00").unwrap());
        assert!(product.inventory_count > 0);
    }
}

#[test]
fn test_inventory_management_workflow() {
    let product_service = ProductService::new();

    // Create product with initial inventory
    let product =
        product_service.create(common::create_test_product("Managed Product", "50.00", 100));

    assert_eq!(product.inventory_count, 100);

    // Simulate sales
    let sale1 = product_service.update_inventory(product.id, 90);
    assert_eq!(sale1.unwrap().inventory_count, 90);

    let sale2 = product_service.update_inventory(product.id, 85);
    assert_eq!(sale2.unwrap().inventory_count, 85);

    // Restock
    let restock = product_service.update_inventory(product.id, 150);
    assert_eq!(restock.unwrap().inventory_count, 150);

    // Verify final state
    let final_product = product_service.get_by_id(product.id).unwrap();
    assert_eq!(final_product.inventory_count, 150);
}

#[test]
fn test_token_expiration_handling() {
    let now = Utc::now();

    // Create expired token
    let expired_claims = Claims {
        sub: "expired_user@example.com".to_string(),
        iat: timestamp_to_usize((now - Duration::hours(48)).timestamp()),
        exp: timestamp_to_usize((now - Duration::hours(24)).timestamp()),
    };

    let expired_token = encode(
        &Header::default(),
        &expired_claims,
        &EncodingKey::from_secret(JWT_TEST_KEY.as_ref()),
    )
    .expect("Failed to create token");

    // Validation should fail for expired token
    let result = validate_token(&expired_token);
    assert!(result.is_err());

    // Valid token should still work
    let valid_token = create_user_token("valid_user@example.com");
    let valid_result = validate_token(&valid_token);
    assert!(valid_result.is_ok());
}

#[test]
fn test_complete_user_journey() {
    // 1. User Registration
    let user_email = "journey@example.com";
    let user_password = "my_password";
    let password_hash = hash(user_password, DEFAULT_COST).unwrap();

    let user = User {
        id: 1,
        email: user_email.to_string(),
        password_hash,
    };

    // 2. User Login
    let login_valid = verify(user_password, &user.password_hash).unwrap();
    assert!(login_valid);

    let auth_token = create_user_token(&user.email);

    // 3. Browse Products
    let product_service = common::create_test_product_service();
    let all_products = product_service.get_all();
    assert!(!all_products.is_empty());

    // 4. Search for specific product
    let mut filter = ProductFilter::new();
    filter.name_contains = Some("iPhone".to_string());
    filter.in_stock = Some(true);
    let search_results = product_service.filter(&filter);
    assert!(!search_results.is_empty());

    // 5. Add to cart
    let selected_product = search_results[0].clone();
    let purchase_quantity = 1;
    let new_inventory = selected_product.inventory_count - purchase_quantity;

    // 6. Checkout
    let claims = validate_token(&auth_token).unwrap();
    assert_eq!(claims.sub, user.email);

    let updated = product_service.update_inventory(selected_product.id, new_inventory);
    assert!(updated.is_some());

    // 7. Verify purchase
    let product_after = product_service.get_by_id(selected_product.id).unwrap();
    assert_eq!(
        product_after.inventory_count,
        selected_product.inventory_count - purchase_quantity
    );
}

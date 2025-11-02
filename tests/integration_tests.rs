//! End-to-end integration tests
//!
//! This module tests complete user flows and integration between components.

mod common;

use common::{auth, cart::TestCartService, get_test_product_service};
use cto_parallel_test::catalog::{ProductFilter, ProductService};
use rust_decimal_macros::dec;

// ============================================================================
// Health Check Tests
// ============================================================================

#[test]
fn test_health_check() {
    // Simulating a health check endpoint
    let product_service = ProductService::new();
    let cart_service = TestCartService::new();

    // Services should be initialized successfully
    assert!(product_service.get_all().is_empty());
    assert!(cart_service.get_cart(1).is_empty());
}

// ============================================================================
// Complete User Shopping Flow Tests
// ============================================================================

#[test]
fn test_complete_shopping_flow() {
    // Setup services
    let product_service = get_test_product_service();
    let cart_service = TestCartService::new();
    let user_id = 1;

    // Step 1: User browses products
    let products = product_service.get_all();
    assert!(!products.is_empty());

    // Step 2: User views product details
    let laptop = products
        .iter()
        .find(|p| p.name == "Laptop Pro")
        .expect("Laptop Pro should exist");
    assert_eq!(laptop.price, dec!(1299.99));
    assert!(laptop.inventory_count > 0);

    // Step 3: User adds product to cart
    cart_service.add_item(user_id, laptop.id, 1);
    let cart = cart_service.get_cart(user_id);
    assert_eq!(cart.len(), 1);
    assert_eq!(cart[0].product_id, laptop.id);

    // Step 4: User adds another product
    let mouse = products
        .iter()
        .find(|p| p.name == "Wireless Mouse")
        .expect("Mouse should exist");
    cart_service.add_item(user_id, mouse.id, 2);
    let cart = cart_service.get_cart(user_id);
    assert_eq!(cart.len(), 2);

    // Step 5: User reviews cart
    let total_items: i32 = cart.iter().map(|item| item.quantity).sum();
    assert_eq!(total_items, 3); // 1 laptop + 2 mice

    // Step 6: User removes an item
    cart_service.remove_item(user_id, mouse.id);
    let cart = cart_service.get_cart(user_id);
    assert_eq!(cart.len(), 1);

    // Step 7: User proceeds to checkout (simulated)
    let final_cart = cart_service.get_cart(user_id);
    assert!(!final_cart.is_empty());
}

#[test]
fn test_shopping_flow_with_filtering() {
    // Setup
    let product_service = get_test_product_service();
    let cart_service = TestCartService::new();
    let user_id = 1;

    // Step 1: User searches for products in a price range
    let filter = ProductFilter::with_price_range(Some(dec!(20.00)), Some(dec!(100.00)));
    let filtered_products = product_service.filter(&filter);

    assert!(!filtered_products.is_empty());
    assert!(filtered_products
        .iter()
        .all(|p| p.price >= dec!(20.00) && p.price <= dec!(100.00)));

    // Step 2: User adds filtered product to cart
    if let Some(product) = filtered_products.first() {
        cart_service.add_item(user_id, product.id, 1);
        let cart = cart_service.get_cart(user_id);
        assert_eq!(cart.len(), 1);
    }
}

#[test]
fn test_user_registration_and_login_flow() {
    // Step 1: User registers
    // In a real implementation, username and email would be stored
    let password = "secure_password_123";

    // Hash password on registration
    let password_hash = auth::hash_test_password(password);

    // Store user (simulated)
    let user_id = 1;

    // Step 2: User logs in
    let login_password = "secure_password_123";
    let login_successful = auth::verify_test_password(login_password, &password_hash);
    assert!(login_successful);

    // Step 3: Create JWT token after successful login
    let token = auth::create_test_token(user_id);
    assert!(!token.is_empty());

    // Step 4: User accesses protected resource with token
    let extracted_user_id = auth::validate_test_token(&token);
    assert!(extracted_user_id.is_ok());
    assert_eq!(extracted_user_id.unwrap(), user_id);
}

#[test]
fn test_authenticated_shopping_flow() {
    // Setup
    let product_service = get_test_product_service();
    let cart_service = TestCartService::new();

    // Step 1: User registers and logs in
    let user_id = 42;
    let password = "mypassword";
    let password_hash = auth::hash_test_password(password);

    // Login
    assert!(auth::verify_test_password(password, &password_hash));
    let token = auth::create_test_token(user_id);

    // Step 2: Validate token for protected cart access
    let validated_user_id = auth::validate_test_token(&token);
    assert!(validated_user_id.is_ok());
    let authenticated_user_id = validated_user_id.unwrap();

    // Step 3: Add items to cart (requires authentication)
    let products = product_service.get_all();
    cart_service.add_item(authenticated_user_id, products[0].id, 1);

    // Step 4: Get cart (requires authentication)
    let cart = cart_service.get_cart(authenticated_user_id);
    assert_eq!(cart.len(), 1);

    // Step 5: Clear cart (requires authentication)
    cart_service.clear_cart(authenticated_user_id);
    let cart = cart_service.get_cart(authenticated_user_id);
    assert_eq!(cart.len(), 0);
}

#[test]
fn test_full_ecommerce_flow() {
    // Complete end-to-end test of the e-commerce system
    let product_service = get_test_product_service();
    let cart_service = TestCartService::new();

    // Step 1: User Registration
    let user_id = 1;
    let password = "secure_pass_123";
    let password_hash = auth::hash_test_password(password);

    // Step 2: User Login
    assert!(auth::verify_test_password(password, &password_hash));
    let token = auth::create_test_token(user_id);
    let authenticated_user = auth::validate_test_token(&token).unwrap();

    // Step 3: Browse Products
    let all_products = product_service.get_all();
    assert_eq!(all_products.len(), 6);

    // Step 4: Filter Products
    let in_stock_filter = ProductFilter::with_stock_status(true);
    let available_products = product_service.filter(&in_stock_filter);
    assert_eq!(available_products.len(), 5); // 1 product is out of stock

    // Step 5: Add Products to Cart
    for (i, product) in available_products.iter().take(3).enumerate() {
        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        let quantity = (i as i32) + 1;
        cart_service.add_item(authenticated_user, product.id, quantity);
    }

    // Step 6: Review Cart
    let cart = cart_service.get_cart(authenticated_user);
    assert_eq!(cart.len(), 3);
    let total_items: i32 = cart.iter().map(|item| item.quantity).sum();
    assert_eq!(total_items, 6); // 1 + 2 + 3

    // Step 7: Update Cart (remove one item)
    let first_item_id = cart[0].product_id;
    cart_service.remove_item(authenticated_user, first_item_id);
    let cart = cart_service.get_cart(authenticated_user);
    assert_eq!(cart.len(), 2);

    // Step 8: Verify Inventory (would be checked at checkout)
    for cart_item in &cart {
        let product = product_service.get_by_id(cart_item.product_id);
        assert!(product.is_some());
        assert!(product.unwrap().inventory_count >= cart_item.quantity);
    }

    // Step 9: Checkout Complete (simulated)
    cart_service.clear_cart(authenticated_user);
    let cart = cart_service.get_cart(authenticated_user);
    assert_eq!(cart.len(), 0);
}

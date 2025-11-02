//! API integration tests
//!
//! This module tests API endpoints for products, cart, and other resources.

mod common;

use common::{create_test_product, get_test_product_service};
use cto_parallel_test::catalog::{NewProduct, ProductFilter, ProductService};
use rust_decimal_macros::dec;

// ============================================================================
// Product CRUD Tests
// ============================================================================

#[test]
fn test_create_product() {
    let service = ProductService::new();
    let new_product = create_test_product("Test Product", dec!(19.99), 10);

    let product = service.create(new_product);

    assert_eq!(product.id, 1);
    assert_eq!(product.name, "Test Product");
    assert_eq!(product.price, dec!(19.99));
    assert_eq!(product.inventory_count, 10);
}

#[test]
fn test_create_multiple_products() {
    let service = ProductService::new();

    let product1 = service.create(create_test_product("Product 1", dec!(10.00), 5));
    let product2 = service.create(create_test_product("Product 2", dec!(20.00), 10));
    let product3 = service.create(create_test_product("Product 3", dec!(30.00), 15));

    assert_eq!(product1.id, 1);
    assert_eq!(product2.id, 2);
    assert_eq!(product3.id, 3);
}

#[test]
fn test_get_all_products() {
    let service = get_test_product_service();
    let products = service.get_all();

    assert_eq!(products.len(), 6);
    assert!(products.iter().any(|p| p.name == "Laptop Pro"));
    assert!(products.iter().any(|p| p.name == "Wireless Mouse"));
}

#[test]
fn test_get_all_products_empty() {
    let service = ProductService::new();
    let products = service.get_all();

    assert_eq!(products.len(), 0);
}

#[test]
fn test_get_product_by_id_found() {
    let service = get_test_product_service();
    let products = service.get_all();
    let first_product_id = products[0].id;

    let result = service.get_by_id(first_product_id);

    assert!(result.is_some());
    let product = result.unwrap();
    assert_eq!(product.id, first_product_id);
}

#[test]
fn test_get_product_by_id_not_found() {
    let service = get_test_product_service();
    let result = service.get_by_id(9999);

    assert!(result.is_none());
}

#[test]
fn test_update_product_inventory() {
    let service = ProductService::new();
    let product = service.create(create_test_product("Test", dec!(10.00), 100));

    let updated = service.update_inventory(product.id, 50);

    assert!(updated.is_some());
    assert_eq!(updated.unwrap().inventory_count, 50);
}

#[test]
fn test_update_inventory_not_found() {
    let service = ProductService::new();
    let result = service.update_inventory(9999, 10);

    assert!(result.is_none());
}

#[test]
fn test_delete_product() {
    let service = ProductService::new();
    let product = service.create(create_test_product("To Delete", dec!(5.00), 1));

    let deleted = service.delete(product.id);

    assert!(deleted);
    assert!(service.get_by_id(product.id).is_none());
}

#[test]
fn test_delete_product_not_found() {
    let service = ProductService::new();
    let deleted = service.delete(9999);

    assert!(!deleted);
}

// ============================================================================
// Product Filtering Tests
// ============================================================================

#[test]
fn test_filter_products_by_name() {
    let service = get_test_product_service();
    let filter = ProductFilter::with_name("laptop");

    let results = service.filter(&filter);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "Laptop Pro");
}

#[test]
fn test_filter_products_by_name_case_insensitive() {
    let service = get_test_product_service();
    let filter = ProductFilter::with_name("MOUSE");

    let results = service.filter(&filter);

    assert_eq!(results.len(), 1);
    assert!(results[0].name.to_lowercase().contains("mouse"));
}

#[test]
fn test_filter_products_by_name_partial_match() {
    let service = get_test_product_service();
    let filter = ProductFilter::with_name("key"); // Should match "Keyboard"

    let results = service.filter(&filter);

    assert_eq!(results.len(), 1);
    assert!(results[0].name.contains("Keyboard"));
}

#[test]
fn test_filter_products_by_min_price() {
    let service = get_test_product_service();
    let filter = ProductFilter::with_price_range(Some(dec!(100.00)), None);

    let results = service.filter(&filter);

    assert!(!results.is_empty());
    assert!(results.iter().all(|p| p.price >= dec!(100.00)));
}

#[test]
fn test_filter_products_by_max_price() {
    let service = get_test_product_service();
    let filter = ProductFilter::with_price_range(None, Some(dec!(50.00)));

    let results = service.filter(&filter);

    assert!(!results.is_empty());
    assert!(results.iter().all(|p| p.price <= dec!(50.00)));
}

#[test]
fn test_filter_products_by_price_range() {
    let service = get_test_product_service();
    let filter = ProductFilter::with_price_range(Some(dec!(50.00)), Some(dec!(200.00)));

    let results = service.filter(&filter);

    assert!(!results.is_empty());
    assert!(results
        .iter()
        .all(|p| p.price >= dec!(50.00) && p.price <= dec!(200.00)));
}

#[test]
fn test_filter_products_in_stock() {
    let service = get_test_product_service();
    let filter = ProductFilter::with_stock_status(true);

    let results = service.filter(&filter);

    assert!(!results.is_empty());
    assert!(results.iter().all(|p| p.inventory_count > 0));
}

#[test]
fn test_filter_products_out_of_stock() {
    let service = get_test_product_service();
    let filter = ProductFilter::with_stock_status(false);

    let results = service.filter(&filter);

    assert_eq!(results.len(), 1); // Webcam HD is out of stock
    assert_eq!(results[0].inventory_count, 0);
}

#[test]
fn test_filter_products_combined() {
    let service = ProductService::new();

    // Create test products
    let _ = service.create(NewProduct {
        name: "Laptop Pro".to_string(),
        description: "High-end laptop".to_string(),
        price: dec!(1200.00),
        inventory_count: 5,
    });

    let _ = service.create(NewProduct {
        name: "Laptop Basic".to_string(),
        description: "Budget laptop".to_string(),
        price: dec!(500.00),
        inventory_count: 0,
    });

    let _ = service.create(NewProduct {
        name: "Desktop".to_string(),
        description: "Gaming desktop".to_string(),
        price: dec!(1500.00),
        inventory_count: 3,
    });

    // Filter: name contains "laptop", price >= 1000, in stock
    let filter = ProductFilter {
        name_contains: Some("laptop".to_string()),
        min_price: Some(dec!(1000.00)),
        max_price: None,
        in_stock: Some(true),
    };

    let results = service.filter(&filter);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "Laptop Pro");
}

#[test]
fn test_filter_products_no_matches() {
    let service = get_test_product_service();
    let filter = ProductFilter::with_name("nonexistent");

    let results = service.filter(&filter);

    assert_eq!(results.len(), 0);
}

#[test]
fn test_filter_products_empty_filter_returns_all() {
    let service = get_test_product_service();
    let filter = ProductFilter::new();

    let results = service.filter(&filter);
    let all_products = service.get_all();

    assert_eq!(results.len(), all_products.len());
}

// ============================================================================
// Error Handling Tests
// ============================================================================

#[test]
fn test_get_product_invalid_id() {
    let service = get_test_product_service();

    // Test with negative ID
    let result = service.get_by_id(-1);
    assert!(result.is_none());

    // Test with zero ID
    let result = service.get_by_id(0);
    assert!(result.is_none());
}

#[test]
fn test_update_inventory_negative_values() {
    let service = ProductService::new();
    let product = service.create(create_test_product("Test", dec!(10.00), 10));

    let updated = service.update_inventory(product.id, -5);

    assert!(updated.is_some());
    assert_eq!(updated.unwrap().inventory_count, -5);
}

#[test]
fn test_create_product_with_zero_price() {
    let service = ProductService::new();
    let product = service.create(create_test_product("Free Item", dec!(0.00), 10));

    assert_eq!(product.price, dec!(0.00));
}

#[test]
fn test_create_product_with_negative_price() {
    let service = ProductService::new();
    let product = service.create(create_test_product("Discount", dec!(-10.00), 5));

    assert_eq!(product.price, dec!(-10.00));
}

#[test]
fn test_create_product_with_zero_inventory() {
    let service = ProductService::new();
    let product = service.create(create_test_product("Out of Stock", dec!(20.00), 0));

    assert_eq!(product.inventory_count, 0);
}

// ============================================================================
// Cart API Tests (using mock cart service)
// ============================================================================

#[test]
fn test_add_item_to_cart() {
    let cart_service = common::cart::TestCartService::new();
    let user_id = 1;
    let product_id = 101;

    cart_service.add_item(user_id, product_id, 2);

    let cart = cart_service.get_cart(user_id);
    assert_eq!(cart.len(), 1);
    assert_eq!(cart[0].product_id, product_id);
    assert_eq!(cart[0].quantity, 2);
}

#[test]
fn test_add_multiple_items_to_cart() {
    let cart_service = common::cart::TestCartService::new();
    let user_id = 1;

    cart_service.add_item(user_id, 101, 1);
    cart_service.add_item(user_id, 102, 2);
    cart_service.add_item(user_id, 103, 3);

    let cart = cart_service.get_cart(user_id);
    assert_eq!(cart.len(), 3);
}

#[test]
fn test_increment_cart_item_quantity() {
    let cart_service = common::cart::TestCartService::new();
    let user_id = 1;
    let product_id = 101;

    cart_service.add_item(user_id, product_id, 1);
    cart_service.add_item(user_id, product_id, 2);

    let cart = cart_service.get_cart(user_id);
    assert_eq!(cart.len(), 1);
    assert_eq!(cart[0].quantity, 3);
}

#[test]
fn test_remove_item_from_cart() {
    let cart_service = common::cart::TestCartService::new();
    let user_id = 1;

    cart_service.add_item(user_id, 101, 1);
    cart_service.add_item(user_id, 102, 2);

    let removed = cart_service.remove_item(user_id, 101);

    assert!(removed);
    let cart = cart_service.get_cart(user_id);
    assert_eq!(cart.len(), 1);
    assert_eq!(cart[0].product_id, 102);
}

#[test]
fn test_remove_nonexistent_item_from_cart() {
    let cart_service = common::cart::TestCartService::new();
    let user_id = 1;

    let removed = cart_service.remove_item(user_id, 999);

    assert!(!removed);
}

#[test]
fn test_clear_cart() {
    let cart_service = common::cart::TestCartService::new();
    let user_id = 1;

    cart_service.add_item(user_id, 101, 1);
    cart_service.add_item(user_id, 102, 2);
    cart_service.add_item(user_id, 103, 3);

    cart_service.clear_cart(user_id);

    let cart = cart_service.get_cart(user_id);
    assert_eq!(cart.len(), 0);
}

#[test]
fn test_get_empty_cart() {
    let cart_service = common::cart::TestCartService::new();
    let user_id = 1;

    let cart = cart_service.get_cart(user_id);

    assert_eq!(cart.len(), 0);
}

#[test]
fn test_multiple_user_carts() {
    let cart_service = common::cart::TestCartService::new();

    cart_service.add_item(1, 101, 1);
    cart_service.add_item(2, 102, 2);
    cart_service.add_item(3, 103, 3);

    let cart1 = cart_service.get_cart(1);
    let cart2 = cart_service.get_cart(2);
    let cart3 = cart_service.get_cart(3);

    assert_eq!(cart1.len(), 1);
    assert_eq!(cart2.len(), 1);
    assert_eq!(cart3.len(), 1);

    assert_eq!(cart1[0].product_id, 101);
    assert_eq!(cart2[0].product_id, 102);
    assert_eq!(cart3[0].product_id, 103);
}

// ============================================================================
// Performance and Concurrency Tests
// ============================================================================

#[test]
fn test_concurrent_product_access() {
    use std::thread;

    let service = ProductService::new();
    let _ = service.create(create_test_product("Test Product", dec!(10.00), 100));

    let service_clone1 = service.clone();
    let service_clone2 = service.clone();

    let handle1 = thread::spawn(move || {
        for _ in 0..10 {
            let _ = service_clone1.get_all();
        }
    });

    let handle2 = thread::spawn(move || {
        for _ in 0..10 {
            let _ = service_clone2.get_all();
        }
    });

    handle1.join().expect("Thread 1 panicked");
    handle2.join().expect("Thread 2 panicked");

    // No assertion needed - test passes if no panic occurs
}

#[test]
fn test_concurrent_product_creation() {
    use std::thread;

    let service = ProductService::new();
    let service_clone1 = service.clone();
    let service_clone2 = service.clone();

    let handle1 = thread::spawn(move || {
        for i in 0..5 {
            let _ = service_clone1.create(create_test_product(
                &format!("Product A{i}"),
                dec!(10.00),
                1,
            ));
        }
    });

    let handle2 = thread::spawn(move || {
        for i in 0..5 {
            let _ = service_clone2.create(create_test_product(
                &format!("Product B{i}"),
                dec!(20.00),
                2,
            ));
        }
    });

    handle1.join().expect("Thread 1 panicked");
    handle2.join().expect("Thread 2 panicked");

    let products = service.get_all();
    assert_eq!(products.len(), 10);
}

// ============================================================================
// Edge Case Tests
// ============================================================================

#[test]
fn test_product_with_very_long_name() {
    let service = ProductService::new();
    let long_name = "A".repeat(1000);
    let product = service.create(create_test_product(&long_name, dec!(10.00), 1));

    assert_eq!(product.name.len(), 1000);
}

#[test]
fn test_product_with_empty_description() {
    let service = ProductService::new();
    let product = service.create(NewProduct {
        name: "Test".to_string(),
        description: String::new(),
        price: dec!(10.00),
        inventory_count: 1,
    });

    assert_eq!(product.description, String::new());
}

#[test]
fn test_filter_with_very_high_price() {
    let service = get_test_product_service();
    let filter = ProductFilter::with_price_range(Some(dec!(10000.00)), None);

    let results = service.filter(&filter);

    assert_eq!(results.len(), 0);
}

#[test]
fn test_price_precision() {
    let service = ProductService::new();
    let product = service.create(create_test_product("Precise", dec!(19.999999), 1));

    assert_eq!(product.price, dec!(19.999999));
}

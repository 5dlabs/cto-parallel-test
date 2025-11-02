//! Catalog API integration tests
//!
//! This module tests the product catalog service functionality including:
//! - Product CRUD operations
//! - Product filtering and search
//! - Inventory management
//! - Error handling

mod common;

use common::{create_custom_product, create_test_product, create_test_product_service};
use cto_parallel_test::catalog::{NewProduct, ProductFilter, ProductService};
use rust_decimal::Decimal;

// ============================================================================
// Product Creation Tests
// ============================================================================

#[test]
fn test_create_product() {
    let service = ProductService::new();
    let new_product = create_test_product("Test Laptop");

    let product = service.create(new_product);

    assert_eq!(product.id, 1);
    assert_eq!(product.name, "Test Laptop");
    assert_eq!(product.price, Decimal::new(9999, 2));
    assert_eq!(product.inventory_count, 100);
}

#[test]
fn test_create_multiple_products() {
    let service = ProductService::new();

    let product1 = service.create(create_test_product("Product 1"));
    let product2 = service.create(create_test_product("Product 2"));
    let product3 = service.create(create_test_product("Product 3"));

    assert_eq!(product1.id, 1);
    assert_eq!(product2.id, 2);
    assert_eq!(product3.id, 3);

    let all_products = service.get_all();
    assert_eq!(all_products.len(), 3);
}

#[test]
fn test_create_product_with_decimal_price() {
    let service = ProductService::new();
    let new_product = NewProduct {
        name: "Expensive Item".to_string(),
        description: "Very expensive".to_string(),
        price: Decimal::new(999_999, 2), // $9999.99
        inventory_count: 1,
    };

    let product = service.create(new_product);
    assert_eq!(product.price, Decimal::new(999_999, 2));
}

#[test]
fn test_create_product_with_zero_inventory() {
    let service = ProductService::new();
    let new_product = create_custom_product("Out of Stock", 5000, 0);

    let product = service.create(new_product);
    assert_eq!(product.inventory_count, 0);
}

// ============================================================================
// Product Retrieval Tests
// ============================================================================

#[test]
fn test_get_all_products_empty() {
    let service = ProductService::new();
    let products = service.get_all();
    assert_eq!(products.len(), 0);
}

#[test]
fn test_get_all_products() {
    let service = create_test_product_service();
    let products = service.get_all();
    assert_eq!(products.len(), 5);
}

#[test]
fn test_get_product_by_id() {
    let service = ProductService::new();
    let created = service.create(create_test_product("Findable Product"));

    let found = service.get_by_id(created.id);
    assert!(found.is_some());

    let product = found.unwrap();
    assert_eq!(product.id, created.id);
    assert_eq!(product.name, "Findable Product");
}

#[test]
fn test_get_product_by_id_not_found() {
    let service = ProductService::new();
    let result = service.get_by_id(999);
    assert!(result.is_none());
}

#[test]
fn test_get_product_by_id_after_creation() {
    let service = create_test_product_service();
    let product = service.get_by_id(1);

    assert!(product.is_some());
    assert_eq!(product.unwrap().name, "Laptop Pro");
}

// ============================================================================
// Inventory Management Tests
// ============================================================================

#[test]
fn test_update_inventory() {
    let service = ProductService::new();
    let product = service.create(create_custom_product("Stock Item", 1000, 100));

    let updated = service.update_inventory(product.id, 50);
    assert!(updated.is_some());
    assert_eq!(updated.unwrap().inventory_count, 50);
}

#[test]
fn test_update_inventory_to_zero() {
    let service = ProductService::new();
    let product = service.create(create_custom_product("Stock Item", 1000, 100));

    let updated = service.update_inventory(product.id, 0);
    assert!(updated.is_some());
    assert_eq!(updated.unwrap().inventory_count, 0);
}

#[test]
fn test_update_inventory_negative() {
    let service = ProductService::new();
    let product = service.create(create_custom_product("Stock Item", 1000, 10));

    let updated = service.update_inventory(product.id, -5);
    assert!(updated.is_some());
    assert_eq!(updated.unwrap().inventory_count, -5);
}

#[test]
fn test_update_inventory_not_found() {
    let service = ProductService::new();
    let result = service.update_inventory(999, 100);
    assert!(result.is_none());
}

#[test]
fn test_update_inventory_persistence() {
    let service = ProductService::new();
    let product = service.create(create_custom_product("Stock Item", 1000, 100));

    let _ = service.update_inventory(product.id, 25);

    let retrieved = service.get_by_id(product.id);
    assert_eq!(retrieved.unwrap().inventory_count, 25);
}

// ============================================================================
// Product Filtering Tests
// ============================================================================

#[test]
fn test_filter_empty_returns_all() {
    let service = create_test_product_service();
    let filter = ProductFilter::new();
    let results = service.filter(&filter);
    assert_eq!(results.len(), 5);
}

#[test]
fn test_filter_by_name() {
    let service = create_test_product_service();
    let filter = ProductFilter::with_name("Mouse");
    let results = service.filter(&filter);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "Wireless Mouse");
}

#[test]
fn test_filter_by_name_case_insensitive() {
    let service = create_test_product_service();
    let filter = ProductFilter::with_name("laptop");
    let results = service.filter(&filter);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "Laptop Pro");
}

#[test]
fn test_filter_by_name_partial_match() {
    let service = create_test_product_service();
    let filter = ProductFilter::with_name("Key");
    let results = service.filter(&filter);

    assert_eq!(results.len(), 1);
    assert!(results[0].name.contains("Keyboard"));
}

#[test]
fn test_filter_by_name_no_match() {
    let service = create_test_product_service();
    let filter = ProductFilter::with_name("Nonexistent");
    let results = service.filter(&filter);

    assert_eq!(results.len(), 0);
}

#[test]
fn test_filter_by_min_price() {
    let service = create_test_product_service();
    let filter = ProductFilter::with_price_range(Some(Decimal::new(10000, 2)), None);
    let results = service.filter(&filter);

    assert_eq!(results.len(), 3); // Laptop, Keyboard, Monitor
    assert!(results.iter().all(|p| p.price >= Decimal::new(10000, 2)));
}

#[test]
fn test_filter_by_max_price() {
    let service = create_test_product_service();
    let filter = ProductFilter::with_price_range(None, Some(Decimal::new(5000, 2)));
    let results = service.filter(&filter);

    assert_eq!(results.len(), 2); // Mouse, USB-C Hub
    assert!(results.iter().all(|p| p.price <= Decimal::new(5000, 2)));
}

#[test]
fn test_filter_by_price_range() {
    let service = create_test_product_service();
    let filter =
        ProductFilter::with_price_range(Some(Decimal::new(5000, 2)), Some(Decimal::new(20000, 2)));
    let results = service.filter(&filter);

    assert_eq!(results.len(), 1); // Keyboard
    assert!(results[0].price >= Decimal::new(5000, 2));
    assert!(results[0].price <= Decimal::new(20000, 2));
}

#[test]
fn test_filter_by_in_stock() {
    let service = create_test_product_service();
    let filter = ProductFilter::with_stock_status(true);
    let results = service.filter(&filter);

    assert_eq!(results.len(), 4); // All except USB-C Hub
    assert!(results.iter().all(|p| p.inventory_count > 0));
}

#[test]
fn test_filter_by_out_of_stock() {
    let service = create_test_product_service();
    let filter = ProductFilter::with_stock_status(false);
    let results = service.filter(&filter);

    assert_eq!(results.len(), 1); // Only USB-C Hub
    assert_eq!(results[0].inventory_count, 0);
}

#[test]
fn test_filter_combined_criteria() {
    let service = create_test_product_service();
    let filter = ProductFilter {
        name_contains: Some("o".to_string()), // Laptop, Monitor, Keyboard, Mouse
        min_price: Some(Decimal::new(10000, 2)), // Laptop, Keyboard, Monitor
        max_price: Some(Decimal::new(20000, 2)), // Keyboard
        in_stock: Some(true),                 // All except USB-C Hub
    };
    let results = service.filter(&filter);

    assert_eq!(results.len(), 1); // Only Keyboard matches all criteria
    assert_eq!(results[0].name, "Mechanical Keyboard");
}

// ============================================================================
// Product Deletion Tests
// ============================================================================

#[test]
fn test_delete_product() {
    let service = ProductService::new();
    let product = service.create(create_test_product("To Delete"));

    let deleted = service.delete(product.id);
    assert!(deleted);

    let found = service.get_by_id(product.id);
    assert!(found.is_none());
}

#[test]
fn test_delete_product_not_found() {
    let service = ProductService::new();
    let deleted = service.delete(999);
    assert!(!deleted);
}

#[test]
fn test_delete_product_reduces_count() {
    let service = create_test_product_service();
    let initial_count = service.get_all().len();

    let _ = service.delete(1);

    let final_count = service.get_all().len();
    assert_eq!(final_count, initial_count - 1);
}

#[test]
fn test_delete_multiple_products() {
    let service = create_test_product_service();

    let deleted1 = service.delete(1);
    let deleted2 = service.delete(2);

    assert!(deleted1);
    assert!(deleted2);
    assert_eq!(service.get_all().len(), 3);
}

// ============================================================================
// Edge Cases and Error Handling
// ============================================================================

#[test]
fn test_product_with_empty_description() {
    let service = ProductService::new();
    let new_product = NewProduct {
        name: "No Description".to_string(),
        description: String::new(),
        price: Decimal::new(1000, 2),
        inventory_count: 10,
    };

    let product = service.create(new_product);
    assert_eq!(product.description, "");
}

#[test]
fn test_product_with_very_long_name() {
    let service = ProductService::new();
    let long_name = "A".repeat(1000);
    let new_product = NewProduct {
        name: long_name.clone(),
        description: "Test".to_string(),
        price: Decimal::new(1000, 2),
        inventory_count: 10,
    };

    let product = service.create(new_product);
    assert_eq!(product.name.len(), 1000);
}

#[test]
fn test_product_with_zero_price() {
    let service = ProductService::new();
    let new_product = create_custom_product("Free Item", 0, 100);

    let product = service.create(new_product);
    assert_eq!(product.price, Decimal::new(0, 2));
}

#[test]
fn test_high_precision_decimal_price() {
    let service = ProductService::new();
    let new_product = NewProduct {
        name: "Precise Price".to_string(),
        description: "Test".to_string(),
        price: Decimal::new(123_456_789, 2), // $1,234,567.89
        inventory_count: 1,
    };

    let product = service.create(new_product);
    assert_eq!(product.price, Decimal::new(123_456_789, 2));
}

// ============================================================================
// Concurrency Tests
// ============================================================================

#[test]
fn test_concurrent_reads() {
    use std::sync::Arc;
    use std::thread;

    let service = Arc::new(create_test_product_service());
    let mut handles = vec![];

    for _ in 0..10 {
        let service_clone = Arc::clone(&service);
        let handle = thread::spawn(move || {
            let products = service_clone.get_all();
            assert_eq!(products.len(), 5);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Thread panicked");
    }
}

#[test]
fn test_concurrent_writes() {
    use std::sync::Arc;
    use std::thread;

    let service = Arc::new(ProductService::new());
    let mut handles = vec![];

    for i in 0..10 {
        let service_clone = Arc::clone(&service);
        let handle = thread::spawn(move || {
            let _ = service_clone.create(create_test_product(&format!("Product {i}")));
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Thread panicked");
    }

    let products = service.get_all();
    assert_eq!(products.len(), 10);

    // Verify all IDs are unique
    let mut ids: Vec<i32> = products.iter().map(|p| p.id).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), 10);
}

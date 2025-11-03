//! Integration tests for Product Catalog API
//!
//! Tests CRUD operations, filtering, and edge cases for the product service.

mod common;

use cto_parallel_test::catalog::models::{NewProduct, ProductFilter};
use cto_parallel_test::catalog::service::ProductService;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;

// Product Creation Tests

#[test]
fn test_create_product() {
    let service = ProductService::new();
    let new_product = common::create_test_product("Test Product", 19.99, 10);

    let product = service.create(new_product);

    assert_eq!(product.id, 1);
    assert_eq!(product.name, "Test Product");
    assert_eq!(product.price, Decimal::from_f64(19.99).unwrap());
    assert_eq!(product.inventory_count, 10);
    assert!(!product.description.is_empty());
}

#[test]
fn test_create_product_auto_incrementing_ids() {
    let service = ProductService::new();

    let product1 = service.create(common::create_test_product("Product 1", 10.0, 5));
    let product2 = service.create(common::create_test_product("Product 2", 20.0, 10));
    let product3 = service.create(common::create_test_product("Product 3", 30.0, 15));

    assert_eq!(product1.id, 1);
    assert_eq!(product2.id, 2);
    assert_eq!(product3.id, 3);
}

#[test]
fn test_create_product_with_zero_inventory() {
    let service = ProductService::new();
    let new_product = common::create_test_product("Out of Stock", 99.99, 0);

    let product = service.create(new_product);

    assert_eq!(product.inventory_count, 0);
}

#[test]
fn test_create_product_with_high_price() {
    let service = ProductService::new();
    let new_product = common::create_test_product("Expensive Item", 9999.99, 1);

    let product = service.create(new_product);

    assert_eq!(product.price, Decimal::from_f64(9999.99).unwrap());
}

// Product Retrieval Tests

#[test]
fn test_get_all_products_empty() {
    let service = ProductService::new();
    let products = service.get_all();

    assert!(products.is_empty());
}

#[test]
fn test_get_all_products() {
    let service = ProductService::new();

    let _ = service.create(common::create_test_product("Product 1", 10.0, 5));
    let _ = service.create(common::create_test_product("Product 2", 20.0, 10));
    let _ = service.create(common::create_test_product("Product 3", 30.0, 15));

    let products = service.get_all();

    assert_eq!(products.len(), 3);
    assert_eq!(products[0].name, "Product 1");
    assert_eq!(products[1].name, "Product 2");
    assert_eq!(products[2].name, "Product 3");
}

#[test]
fn test_get_product_by_id_found() {
    let service = ProductService::new();
    let created = service.create(common::create_test_product("Test Product", 15.0, 8));

    let found = service.get_by_id(created.id);

    assert!(found.is_some());
    let product = found.unwrap();
    assert_eq!(product.id, created.id);
    assert_eq!(product.name, "Test Product");
    assert_eq!(product.price, Decimal::from_f64(15.0).unwrap());
}

#[test]
fn test_get_product_by_id_not_found() {
    let service = ProductService::new();

    let result = service.get_by_id(999);

    assert!(result.is_none());
}

#[test]
fn test_get_product_by_id_negative_id() {
    let service = ProductService::new();

    let result = service.get_by_id(-1);

    assert!(result.is_none());
}

// Product Update Tests

#[test]
fn test_update_inventory_success() {
    let service = ProductService::new();
    let product = service.create(common::create_test_product("Test Product", 10.0, 5));

    let updated = service.update_inventory(product.id, 20);

    assert!(updated.is_some());
    let updated_product = updated.unwrap();
    assert_eq!(updated_product.inventory_count, 20);

    // Verify persistence
    let retrieved = service.get_by_id(product.id).unwrap();
    assert_eq!(retrieved.inventory_count, 20);
}

#[test]
fn test_update_inventory_to_zero() {
    let service = ProductService::new();
    let product = service.create(common::create_test_product("Test Product", 10.0, 5));

    let updated = service.update_inventory(product.id, 0);

    assert!(updated.is_some());
    assert_eq!(updated.unwrap().inventory_count, 0);
}

#[test]
fn test_update_inventory_not_found() {
    let service = ProductService::new();

    let result = service.update_inventory(999, 10);

    assert!(result.is_none());
}

#[test]
fn test_update_inventory_increase_and_decrease() {
    let service = ProductService::new();
    let product = service.create(common::create_test_product("Test Product", 10.0, 10));

    // Increase inventory
    let updated1 = service.update_inventory(product.id, 20);
    assert_eq!(updated1.unwrap().inventory_count, 20);

    // Decrease inventory
    let updated2 = service.update_inventory(product.id, 5);
    assert_eq!(updated2.unwrap().inventory_count, 5);
}

// Product Deletion Tests

#[test]
fn test_delete_product_success() {
    let service = ProductService::new();
    let product = service.create(common::create_test_product("Test Product", 10.0, 5));

    let deleted = service.delete(product.id);

    assert!(deleted);
    assert!(service.get_by_id(product.id).is_none());
    assert_eq!(service.get_all().len(), 0);
}

#[test]
fn test_delete_product_not_found() {
    let service = ProductService::new();

    let deleted = service.delete(999);

    assert!(!deleted);
}

#[test]
fn test_delete_product_multiple() {
    let service = ProductService::new();
    let p1 = service.create(common::create_test_product("Product 1", 10.0, 5));
    let p2 = service.create(common::create_test_product("Product 2", 20.0, 10));
    let p3 = service.create(common::create_test_product("Product 3", 30.0, 15));

    assert!(service.delete(p2.id));

    let products = service.get_all();
    assert_eq!(products.len(), 2);
    assert_eq!(products[0].id, p1.id);
    assert_eq!(products[1].id, p3.id);
}

// Product Filtering Tests

#[test]
fn test_filter_empty_filter_returns_all() {
    let service = common::create_test_product_service();
    let filter = ProductFilter::new();

    let results = service.filter(&filter);

    assert_eq!(results.len(), 3);
}

#[test]
fn test_filter_by_name() {
    let service = ProductService::new();
    let _ = service.create(common::create_test_product("Apple iPhone", 999.0, 10));
    let _ = service.create(common::create_test_product("Samsung Galaxy", 899.0, 15));
    let _ = service.create(common::create_test_product("Apple MacBook", 1999.0, 5));

    let filter = ProductFilter {
        name_contains: Some("apple".to_string()),
        min_price: None,
        max_price: None,
        in_stock: None,
    };

    let results = service.filter(&filter);

    assert_eq!(results.len(), 2);
    assert!(results
        .iter()
        .all(|p| p.name.to_lowercase().contains("apple")));
}

#[test]
fn test_filter_by_name_case_insensitive() {
    let service = common::create_test_product_service();

    let filter = ProductFilter {
        name_contains: Some("LAPTOP".to_string()),
        min_price: None,
        max_price: None,
        in_stock: None,
    };

    let results = service.filter(&filter);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "Laptop");
}

#[test]
fn test_filter_by_name_no_matches() {
    let service = common::create_test_product_service();

    let filter = ProductFilter {
        name_contains: Some("NonExistent".to_string()),
        min_price: None,
        max_price: None,
        in_stock: None,
    };

    let results = service.filter(&filter);

    assert!(results.is_empty());
}

#[test]
fn test_filter_by_min_price() {
    let service = common::create_test_product_service();

    let filter = ProductFilter {
        name_contains: None,
        min_price: Some(common::decimal_from_str("50.0")),
        max_price: None,
        in_stock: None,
    };

    let results = service.filter(&filter);

    assert_eq!(results.len(), 2); // Laptop (999.99) and Keyboard (79.99)
    assert!(results
        .iter()
        .all(|p| p.price >= common::decimal_from_str("50.0")));
}

#[test]
fn test_filter_by_max_price() {
    let service = common::create_test_product_service();

    let filter = ProductFilter {
        name_contains: None,
        min_price: None,
        max_price: Some(common::decimal_from_str("50.0")),
        in_stock: None,
    };

    let results = service.filter(&filter);

    assert_eq!(results.len(), 1); // Mouse (29.99)
    assert!(results
        .iter()
        .all(|p| p.price <= common::decimal_from_str("50.0")));
}

#[test]
fn test_filter_by_price_range() {
    let service = ProductService::new();
    let _ = service.create(common::create_test_product("Cheap", 10.0, 10));
    let _ = service.create(common::create_test_product("Mid", 50.0, 10));
    let _ = service.create(common::create_test_product("Expensive", 100.0, 10));

    let filter = ProductFilter {
        name_contains: None,
        min_price: Some(common::decimal_from_str("20.0")),
        max_price: Some(common::decimal_from_str("80.0")),
        in_stock: None,
    };

    let results = service.filter(&filter);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "Mid");
}

#[test]
fn test_filter_by_in_stock() {
    let service = ProductService::new();
    let _ = service.create(common::create_test_product("In Stock 1", 10.0, 5));
    let _ = service.create(common::create_test_product("Out of Stock", 20.0, 0));
    let _ = service.create(common::create_test_product("In Stock 2", 30.0, 1));

    let filter = ProductFilter {
        name_contains: None,
        min_price: None,
        max_price: None,
        in_stock: Some(true),
    };

    let results = service.filter(&filter);

    assert_eq!(results.len(), 2);
    assert!(results.iter().all(|p| p.inventory_count > 0));
}

#[test]
fn test_filter_by_out_of_stock() {
    let service = ProductService::new();
    let _ = service.create(common::create_test_product("In Stock", 10.0, 5));
    let _ = service.create(common::create_test_product("Out of Stock", 20.0, 0));

    let filter = ProductFilter {
        name_contains: None,
        min_price: None,
        max_price: None,
        in_stock: Some(false),
    };

    let results = service.filter(&filter);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].inventory_count, 0);
}

#[test]
fn test_filter_combined_criteria() {
    let service = ProductService::new();
    let _ = service.create(common::create_test_product("Apple iPhone", 999.0, 10));
    let _ = service.create(common::create_test_product("Apple Watch", 399.0, 0));
    let _ = service.create(common::create_test_product("Apple MacBook", 1999.0, 5));
    let _ = service.create(common::create_test_product("Samsung Galaxy", 899.0, 15));

    let filter = ProductFilter {
        name_contains: Some("apple".to_string()),
        min_price: Some(common::decimal_from_str("500.0")),
        max_price: Some(common::decimal_from_str("1500.0")),
        in_stock: Some(true),
    };

    let results = service.filter(&filter);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "Apple iPhone");
    assert!(results[0].price >= common::decimal_from_str("500.0"));
    assert!(results[0].price <= common::decimal_from_str("1500.0"));
    assert!(results[0].inventory_count > 0);
}

// Edge Cases and Error Handling

#[test]
fn test_product_with_decimal_precision() {
    let service = ProductService::new();
    let new_product = NewProduct {
        name: "Precise Product".to_string(),
        description: "Test precision".to_string(),
        price: common::decimal_from_str("19.99"),
        inventory_count: 10,
    };

    let product = service.create(new_product);

    assert_eq!(product.price.to_string(), "19.99");
}

#[test]
fn test_product_with_large_inventory() {
    let service = ProductService::new();
    let new_product = common::create_test_product("Bulk Item", 0.99, 1_000_000);

    let product = service.create(new_product);

    assert_eq!(product.inventory_count, 1_000_000);
}

#[test]
fn test_service_is_thread_safe() {
    use std::thread;

    let service = ProductService::new();
    let service_clone1 = service.clone();
    let service_clone2 = service.clone();

    let handle1 = thread::spawn(move || {
        for i in 0..10 {
            let _ = service_clone1.create(common::create_test_product(
                &format!("Thread1-{i}"),
                10.0,
                5,
            ));
        }
    });

    let handle2 = thread::spawn(move || {
        for i in 0..10 {
            let _ = service_clone2.create(common::create_test_product(
                &format!("Thread2-{i}"),
                20.0,
                10,
            ));
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let products = service.get_all();
    assert_eq!(products.len(), 20);

    // Verify all IDs are unique
    let mut ids: Vec<i32> = products.iter().map(|p| p.id).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), 20);
}

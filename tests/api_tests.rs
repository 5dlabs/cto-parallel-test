mod common;

use cto_parallel_test::catalog::{NewProduct, ProductFilter, ProductService};
use rust_decimal::Decimal;
use std::str::FromStr;

#[test]
fn test_product_service_create() {
    let service = ProductService::new();

    let new_product = NewProduct {
        name: "Test Laptop".to_string(),
        description: "High-performance laptop".to_string(),
        price: Decimal::from_str("1299.99").unwrap(),
        inventory_count: 25,
    };

    let created = service.create(new_product);

    assert_eq!(created.id, 1);
    assert_eq!(created.name, "Test Laptop");
    assert_eq!(created.description, "High-performance laptop");
    assert_eq!(created.price, Decimal::from_str("1299.99").unwrap());
    assert_eq!(created.inventory_count, 25);
}

#[test]
fn test_product_service_get_all() {
    let service = common::create_test_product_service();

    let products = service.get_all();

    assert_eq!(products.len(), 4);
    assert!(products.iter().any(|p| p.name == "Apple iPhone 14"));
    assert!(products.iter().any(|p| p.name == "Samsung Galaxy S23"));
}

#[test]
fn test_product_service_get_by_id() {
    let service = common::create_minimal_product_service();

    let products = service.get_all();
    let first_product_id = products[0].id;

    let found = service.get_by_id(first_product_id);
    assert!(found.is_some());
    assert_eq!(found.unwrap().name, "Test Product");
}

#[test]
fn test_product_service_get_by_id_not_found() {
    let service = ProductService::new();

    let not_found = service.get_by_id(999);

    assert!(not_found.is_none());
}

#[test]
fn test_product_service_update_inventory() {
    let service = common::create_minimal_product_service();

    let products = service.get_all();
    let product_id = products[0].id;

    let updated = service.update_inventory(product_id, 50);

    assert!(updated.is_some());
    assert_eq!(updated.unwrap().inventory_count, 50);

    let retrieved = service.get_by_id(product_id).unwrap();
    assert_eq!(retrieved.inventory_count, 50);
}

#[test]
fn test_product_service_update_inventory_not_found() {
    let service = ProductService::new();

    let result = service.update_inventory(999, 10);

    assert!(result.is_none());
}

#[test]
fn test_product_service_delete() {
    let service = common::create_minimal_product_service();

    let products = service.get_all();
    let product_id = products[0].id;

    let deleted = service.delete(product_id);

    assert!(deleted);

    let found = service.get_by_id(product_id);
    assert!(found.is_none());

    let remaining = service.get_all();
    assert_eq!(remaining.len(), 0);
}

#[test]
fn test_product_service_delete_not_found() {
    let service = ProductService::new();

    let deleted = service.delete(999);

    assert!(!deleted);
}

#[test]
fn test_product_filter_by_name() {
    let service = common::create_test_product_service();

    let mut filter = ProductFilter::new();
    filter.name_contains = Some("Apple".to_string());

    let results = service.filter(&filter);

    assert_eq!(results.len(), 2);
    assert!(results.iter().all(|p| p.name.contains("Apple")));
}

#[test]
fn test_product_filter_by_name_case_insensitive() {
    let service = common::create_test_product_service();

    let mut filter = ProductFilter::new();
    filter.name_contains = Some("apple".to_string());

    let results = service.filter(&filter);

    assert_eq!(results.len(), 2);
}

#[test]
fn test_product_filter_by_min_price() {
    let service = common::create_test_product_service();

    let mut filter = ProductFilter::new();
    filter.min_price = Some(Decimal::from_str("500.00").unwrap());

    let results = service.filter(&filter);

    assert!(results.len() >= 2);
    assert!(results
        .iter()
        .all(|p| p.price >= Decimal::from_str("500.00").unwrap()));
}

#[test]
fn test_product_filter_by_max_price() {
    let service = common::create_test_product_service();

    let mut filter = ProductFilter::new();
    filter.max_price = Some(Decimal::from_str("400.00").unwrap());

    let results = service.filter(&filter);

    assert!(results
        .iter()
        .all(|p| p.price <= Decimal::from_str("400.00").unwrap()));
}

#[test]
fn test_product_filter_by_price_range() {
    let service = common::create_test_product_service();

    let mut filter = ProductFilter::new();
    filter.min_price = Some(Decimal::from_str("300.00").unwrap());
    filter.max_price = Some(Decimal::from_str("500.00").unwrap());

    let results = service.filter(&filter);

    assert!(results.iter().all(|p| {
        p.price >= Decimal::from_str("300.00").unwrap()
            && p.price <= Decimal::from_str("500.00").unwrap()
    }));
}

#[test]
fn test_product_filter_in_stock() {
    let service = common::create_test_product_service();

    let mut filter = ProductFilter::new();
    filter.in_stock = Some(true);

    let results = service.filter(&filter);

    assert!(results.iter().all(|p| p.inventory_count > 0));
}

#[test]
fn test_product_filter_out_of_stock() {
    let service = common::create_test_product_service();

    let mut filter = ProductFilter::new();
    filter.in_stock = Some(false);

    let results = service.filter(&filter);

    assert!(results.iter().all(|p| p.inventory_count == 0));
    assert!(results.iter().any(|p| p.name == "Sony WH-1000XM5"));
}

#[test]
fn test_product_filter_combined() {
    let service = common::create_test_product_service();

    let mut filter = ProductFilter::new();
    filter.name_contains = Some("Apple".to_string());
    filter.min_price = Some(Decimal::from_str("900.00").unwrap());
    filter.in_stock = Some(true);

    let results = service.filter(&filter);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "Apple iPhone 14");
}

#[test]
fn test_product_filter_no_matches() {
    let service = common::create_test_product_service();

    let mut filter = ProductFilter::new();
    filter.name_contains = Some("NonExistent".to_string());

    let results = service.filter(&filter);

    assert_eq!(results.len(), 0);
}

#[test]
fn test_product_filter_empty_filter() {
    let service = common::create_test_product_service();

    let filter = ProductFilter::new();

    let results = service.filter(&filter);

    assert_eq!(results.len(), 4);
}

#[test]
fn test_product_crud_operations() {
    let service = ProductService::new();

    // Create
    let new_product = common::create_test_product("Gaming Mouse", "79.99", 15);
    let created = service.create(new_product);
    assert_eq!(created.name, "Gaming Mouse");

    // Read
    let found = service.get_by_id(created.id);
    assert!(found.is_some());
    assert_eq!(found.unwrap().name, "Gaming Mouse");

    // Update
    let updated = service.update_inventory(created.id, 25);
    assert!(updated.is_some());
    assert_eq!(updated.unwrap().inventory_count, 25);

    // Delete
    let deleted = service.delete(created.id);
    assert!(deleted);

    let not_found = service.get_by_id(created.id);
    assert!(not_found.is_none());
}

#[test]
fn test_multiple_products_different_ids() {
    let service = ProductService::new();

    let product1 = service.create(common::create_test_product("Product 1", "10.00", 5));
    let product2 = service.create(common::create_test_product("Product 2", "20.00", 10));
    let product3 = service.create(common::create_test_product("Product 3", "30.00", 15));

    assert_eq!(product1.id, 1);
    assert_eq!(product2.id, 2);
    assert_eq!(product3.id, 3);

    let all = service.get_all();
    assert_eq!(all.len(), 3);
}

#[test]
fn test_inventory_edge_cases() {
    let service = ProductService::new();

    // Zero inventory
    let product = service.create(common::create_test_product("Out of Stock", "25.00", 0));
    assert_eq!(product.inventory_count, 0);

    // Negative inventory (returns/overselling)
    let updated = service.update_inventory(product.id, -5);
    assert!(updated.is_some());
    assert_eq!(updated.unwrap().inventory_count, -5);

    // Large inventory
    let large = service.update_inventory(product.id, 1_000_000);
    assert!(large.is_some());
    assert_eq!(large.unwrap().inventory_count, 1_000_000);
}

#[test]
fn test_price_precision() {
    let service = ProductService::new();

    let product = service.create(NewProduct {
        name: "Precision Test".to_string(),
        description: "Testing decimal precision".to_string(),
        price: Decimal::from_str("19.999").unwrap(),
        inventory_count: 1,
    });

    assert_eq!(product.price.to_string(), "19.999");

    let retrieved = service.get_by_id(product.id).unwrap();
    assert_eq!(retrieved.price, Decimal::from_str("19.999").unwrap());
}

#[test]
fn test_error_handling_invalid_id() {
    let service = common::create_test_product_service();

    // Test get with invalid ID
    assert!(service.get_by_id(0).is_none());
    assert!(service.get_by_id(-1).is_none());
    assert!(service.get_by_id(999_999).is_none());

    // Test update with invalid ID
    assert!(service.update_inventory(0, 10).is_none());
    assert!(service.update_inventory(-1, 10).is_none());

    // Test delete with invalid ID
    assert!(!service.delete(0));
    assert!(!service.delete(-1));
}

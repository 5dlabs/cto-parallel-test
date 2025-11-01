//! API endpoint tests for the product catalog.
//!
//! Tests cover:
//! - Product CRUD operations
//! - Product filtering by name, price, and stock
//! - Error handling and edge cases
//! - Inventory management

mod common;

use common::{get_empty_product_service, get_test_product_service};
use cto_parallel_test::catalog::{NewProduct, ProductFilter};
use rust_decimal::Decimal;
use std::str::FromStr;

// ============================================================================
// Product Creation Tests
// ============================================================================

#[test]
fn test_create_product_success() {
    let service = get_empty_product_service();

    let new_product = NewProduct {
        name: "Test Product".to_string(),
        description: "Test Description".to_string(),
        price: Decimal::from_str("19.99").unwrap(),
        inventory_count: 100,
    };

    let product = service.create(new_product);

    assert_eq!(product.id, 1);
    assert_eq!(product.name, "Test Product");
    assert_eq!(product.description, "Test Description");
    assert_eq!(product.price, Decimal::from_str("19.99").unwrap());
    assert_eq!(product.inventory_count, 100);
}

#[test]
fn test_create_multiple_products_increments_ids() {
    let service = get_empty_product_service();

    let product1 = service.create(NewProduct {
        name: "Product 1".to_string(),
        description: "Description 1".to_string(),
        price: Decimal::from_str("10.00").unwrap(),
        inventory_count: 10,
    });

    let product2 = service.create(NewProduct {
        name: "Product 2".to_string(),
        description: "Description 2".to_string(),
        price: Decimal::from_str("20.00").unwrap(),
        inventory_count: 20,
    });

    assert_eq!(product1.id, 1);
    assert_eq!(product2.id, 2);
}

#[test]
fn test_create_product_with_zero_inventory() {
    let service = get_empty_product_service();

    let product = service.create(NewProduct {
        name: "Out of Stock".to_string(),
        description: "Currently unavailable".to_string(),
        price: Decimal::from_str("50.00").unwrap(),
        inventory_count: 0,
    });

    assert_eq!(product.inventory_count, 0);
}

#[test]
fn test_create_product_with_decimal_price() {
    let service = get_empty_product_service();

    let product = service.create(NewProduct {
        name: "Precise Price".to_string(),
        description: "Exact pricing".to_string(),
        price: Decimal::from_str("123.456").unwrap(),
        inventory_count: 5,
    });

    assert_eq!(product.price, Decimal::from_str("123.456").unwrap());
}

// ============================================================================
// Product Retrieval Tests
// ============================================================================

#[test]
fn test_get_all_products_empty_service() {
    let service = get_empty_product_service();
    let products = service.get_all();

    assert_eq!(products.len(), 0);
}

#[test]
fn test_get_all_products_with_data() {
    let service = get_test_product_service();
    let products = service.get_all();

    assert_eq!(products.len(), 4);
}

#[test]
fn test_get_product_by_id_exists() {
    let service = get_test_product_service();

    let product = service.get_by_id(1);

    assert!(product.is_some());
    let product = product.unwrap();
    assert_eq!(product.id, 1);
    assert_eq!(product.name, "Laptop");
}

#[test]
fn test_get_product_by_id_not_exists() {
    let service = get_test_product_service();

    let product = service.get_by_id(999);

    assert!(product.is_none());
}

#[test]
fn test_get_product_by_id_negative_id() {
    let service = get_test_product_service();

    let product = service.get_by_id(-1);

    assert!(product.is_none());
}

// ============================================================================
// Product Filtering Tests
// ============================================================================

#[test]
fn test_filter_products_by_name() {
    let service = get_test_product_service();

    let filter = ProductFilter {
        name_contains: Some("Laptop".to_string()),
        ..Default::default()
    };

    let filtered = service.filter(&filter);

    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].name, "Laptop");
}

#[test]
fn test_filter_products_by_name_case_insensitive() {
    let service = get_test_product_service();

    let filter = ProductFilter {
        name_contains: Some("laptop".to_string()),
        ..Default::default()
    };

    let filtered = service.filter(&filter);

    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].name, "Laptop");
}

#[test]
fn test_filter_products_by_name_partial_match() {
    let service = get_test_product_service();

    let filter = ProductFilter {
        name_contains: Some("o".to_string()),
        ..Default::default()
    };

    let filtered = service.filter(&filter);

    // Should match: Laptop, Mouse, Keyboard, Monitor (all contain 'o')
    assert_eq!(filtered.len(), 4);
}

#[test]
fn test_filter_products_by_min_price() {
    let service = get_test_product_service();

    let filter = ProductFilter {
        min_price: Some(Decimal::from_str("100.00").unwrap()),
        ..Default::default()
    };

    let filtered = service.filter(&filter);

    // Should match: Laptop (999.99), Monitor (499.99)
    assert_eq!(filtered.len(), 2);
    assert!(filtered
        .iter()
        .all(|p| p.price >= Decimal::from_str("100.00").unwrap()));
}

#[test]
fn test_filter_products_by_max_price() {
    let service = get_test_product_service();

    let filter = ProductFilter {
        max_price: Some(Decimal::from_str("100.00").unwrap()),
        ..Default::default()
    };

    let filtered = service.filter(&filter);

    // Should match: Mouse (29.99), Keyboard (79.99)
    assert_eq!(filtered.len(), 2);
    assert!(filtered
        .iter()
        .all(|p| p.price <= Decimal::from_str("100.00").unwrap()));
}

#[test]
fn test_filter_products_by_price_range() {
    let service = get_test_product_service();

    let filter = ProductFilter {
        min_price: Some(Decimal::from_str("50.00").unwrap()),
        max_price: Some(Decimal::from_str("500.00").unwrap()),
        ..Default::default()
    };

    let filtered = service.filter(&filter);

    // Should match: Keyboard (79.99), Monitor (499.99)
    assert_eq!(filtered.len(), 2);
    assert!(filtered.iter().all(|p| {
        p.price >= Decimal::from_str("50.00").unwrap()
            && p.price <= Decimal::from_str("500.00").unwrap()
    }));
}

#[test]
fn test_filter_products_in_stock() {
    let service = get_test_product_service();

    let filter = ProductFilter {
        in_stock: Some(true),
        ..Default::default()
    };

    let filtered = service.filter(&filter);

    // Should match: Laptop, Mouse, Monitor (all with inventory > 0)
    assert_eq!(filtered.len(), 3);
    assert!(filtered.iter().all(|p| p.inventory_count > 0));
}

#[test]
fn test_filter_products_out_of_stock() {
    let service = get_test_product_service();

    let filter = ProductFilter {
        in_stock: Some(false),
        ..Default::default()
    };

    let filtered = service.filter(&filter);

    // Should match: Keyboard (inventory = 0)
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].name, "Keyboard");
    assert_eq!(filtered[0].inventory_count, 0);
}

#[test]
fn test_filter_products_multiple_criteria() {
    let service = get_test_product_service();

    let filter = ProductFilter {
        name_contains: Some("o".to_string()),
        min_price: Some(Decimal::from_str("40.00").unwrap()),
        in_stock: Some(true),
        ..Default::default()
    };

    let filtered = service.filter(&filter);

    // Should match: Laptop (has 'o', price > 40, in stock) and Monitor (has 'o', price > 40, in stock)
    assert_eq!(filtered.len(), 2);
    assert!(filtered.iter().any(|p| p.name == "Laptop"));
    assert!(filtered.iter().any(|p| p.name == "Monitor"));
}

#[test]
fn test_filter_products_no_matches() {
    let service = get_test_product_service();

    let filter = ProductFilter {
        name_contains: Some("NonExistent".to_string()),
        ..Default::default()
    };

    let filtered = service.filter(&filter);

    assert_eq!(filtered.len(), 0);
}

#[test]
fn test_filter_products_empty_filter() {
    let service = get_test_product_service();

    let filter = ProductFilter::new();
    let filtered = service.filter(&filter);

    // Empty filter should return all products
    assert_eq!(filtered.len(), 4);
}

// ============================================================================
// Inventory Management Tests
// ============================================================================

#[test]
fn test_update_inventory_success() {
    let service = get_test_product_service();

    let result = service.update_inventory(1, 20);

    assert!(result.is_some());
    let product = service.get_by_id(1).unwrap();
    assert_eq!(product.inventory_count, 20);
}

#[test]
fn test_update_inventory_to_zero() {
    let service = get_test_product_service();

    let result = service.update_inventory(1, 0);

    assert!(result.is_some());
    let product = service.get_by_id(1).unwrap();
    assert_eq!(product.inventory_count, 0);
}

#[test]
fn test_update_inventory_nonexistent_product() {
    let service = get_test_product_service();

    let result = service.update_inventory(999, 100);

    assert!(result.is_none());
}

#[test]
fn test_update_inventory_increase() {
    let service = get_test_product_service();

    let original = service.get_by_id(2).unwrap();
    let original_count = original.inventory_count;

    let _ = service.update_inventory(2, original_count + 10);

    let updated = service.get_by_id(2).unwrap();
    assert_eq!(updated.inventory_count, original_count + 10);
}

#[test]
fn test_update_inventory_decrease() {
    let service = get_test_product_service();

    let _ = service.update_inventory(2, 5);

    let updated = service.get_by_id(2).unwrap();
    assert_eq!(updated.inventory_count, 5);
}

// ============================================================================
// Thread Safety Tests
// ============================================================================

#[test]
fn test_service_can_be_cloned() {
    let service = get_test_product_service();
    let cloned_service = service.clone();

    let products1 = service.get_all();
    let products2 = cloned_service.get_all();

    assert_eq!(products1.len(), products2.len());
    assert_eq!(products1, products2);
}

#[test]
fn test_service_shares_state_after_clone() {
    let service = get_test_product_service();
    let cloned_service = service.clone();

    // Add product using cloned service
    let _ = cloned_service.create(NewProduct {
        name: "New Product".to_string(),
        description: "Added via clone".to_string(),
        price: Decimal::from_str("99.99").unwrap(),
        inventory_count: 10,
    });

    // Original service should see the new product
    let products = service.get_all();
    assert_eq!(products.len(), 5);
    assert!(products.iter().any(|p| p.name == "New Product"));
}

// ============================================================================
// Edge Cases and Error Handling Tests
// ============================================================================

#[test]
fn test_create_product_with_empty_name() {
    let service = get_empty_product_service();

    let product = service.create(NewProduct {
        name: String::new(),
        description: "No name".to_string(),
        price: Decimal::from_str("10.00").unwrap(),
        inventory_count: 10,
    });

    assert_eq!(product.name, "");
}

#[test]
fn test_create_product_with_empty_description() {
    let service = get_empty_product_service();

    let product = service.create(NewProduct {
        name: "Product".to_string(),
        description: String::new(),
        price: Decimal::from_str("10.00").unwrap(),
        inventory_count: 10,
    });

    assert_eq!(product.description, "");
}

#[test]
fn test_create_product_with_very_high_price() {
    let service = get_empty_product_service();

    let product = service.create(NewProduct {
        name: "Expensive".to_string(),
        description: "Very expensive item".to_string(),
        price: Decimal::from_str("999999999.99").unwrap(),
        inventory_count: 1,
    });

    assert_eq!(product.price, Decimal::from_str("999999999.99").unwrap());
}

#[test]
fn test_create_product_with_zero_price() {
    let service = get_empty_product_service();

    let product = service.create(NewProduct {
        name: "Free Item".to_string(),
        description: "Free product".to_string(),
        price: Decimal::from_str("0.00").unwrap(),
        inventory_count: 100,
    });

    assert_eq!(product.price, Decimal::from_str("0.00").unwrap());
}

#[test]
fn test_filter_with_impossible_price_range() {
    let service = get_test_product_service();

    let filter = ProductFilter {
        min_price: Some(Decimal::from_str("1000.00").unwrap()),
        max_price: Some(Decimal::from_str("10.00").unwrap()),
        ..Default::default()
    };

    let filtered = service.filter(&filter);

    assert_eq!(filtered.len(), 0);
}

#[test]
fn test_large_number_of_products() {
    let service = get_empty_product_service();

    // Create 1000 products
    for i in 0..1000 {
        let _ = service.create(NewProduct {
            name: format!("Product {i}"),
            description: format!("Description {i}"),
            price: Decimal::from_str("10.00").unwrap(),
            inventory_count: i,
        });
    }

    let products = service.get_all();
    assert_eq!(products.len(), 1000);

    // Verify IDs are sequential
    for (idx, product) in products.iter().enumerate() {
        assert_eq!(product.id, i32::try_from(idx + 1).unwrap());
    }
}

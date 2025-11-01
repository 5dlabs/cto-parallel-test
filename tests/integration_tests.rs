//! End-to-end integration tests covering complete user flows.
//!
//! Tests cover:
//! - Complete shopping flow scenarios
//! - Multi-step user interactions
//! - Component integration verification
//! - Real-world usage patterns

mod common;

use common::get_test_product_service;
use cto_parallel_test::catalog::{NewProduct, ProductFilter};
use rust_decimal::Decimal;
use std::str::FromStr;

// ============================================================================
// Complete Shopping Flow Tests
// ============================================================================

#[test]
fn test_complete_shopping_flow_browse_and_filter() {
    // Simulate a user browsing products and filtering by criteria
    let service = get_test_product_service();

    // Step 1: User browses all products
    let all_products = service.get_all();
    assert_eq!(all_products.len(), 4);

    // Step 2: User filters by price range (affordable items)
    let affordable_filter = ProductFilter {
        max_price: Some(Decimal::from_str("100.00").unwrap()),
        ..Default::default()
    };
    let affordable_products = service.filter(&affordable_filter);
    assert_eq!(affordable_products.len(), 2); // Mouse and Keyboard

    // Step 3: User further filters to only in-stock items
    let in_stock_filter = ProductFilter {
        max_price: Some(Decimal::from_str("100.00").unwrap()),
        in_stock: Some(true),
        ..Default::default()
    };
    let available_products = service.filter(&in_stock_filter);
    assert_eq!(available_products.len(), 1); // Only Mouse

    // Step 4: User selects a specific product
    let selected_product = available_products[0].clone();
    assert_eq!(selected_product.name, "Mouse");
    assert_eq!(selected_product.price, Decimal::from_str("29.99").unwrap());
    assert!(selected_product.inventory_count > 0);
}

#[test]
fn test_complete_shopping_flow_search_and_purchase() {
    // Simulate a user searching for a specific product and "purchasing" it
    let service = get_test_product_service();

    // Step 1: User searches for "Laptop"
    let search_filter = ProductFilter {
        name_contains: Some("Laptop".to_string()),
        ..Default::default()
    };
    let search_results = service.filter(&search_filter);
    assert_eq!(search_results.len(), 1);

    let laptop = &search_results[0];
    assert_eq!(laptop.name, "Laptop");

    // Step 2: Check product availability
    assert!(laptop.inventory_count > 0, "Product should be in stock");
    let original_inventory = laptop.inventory_count;

    // Step 3: "Purchase" the product (decrease inventory)
    let purchase_quantity = 2;
    let result = service.update_inventory(laptop.id, original_inventory - purchase_quantity);
    assert!(result.is_some(), "Inventory update should succeed");

    // Step 4: Verify inventory was updated
    let updated_product = service.get_by_id(laptop.id).unwrap();
    assert_eq!(
        updated_product.inventory_count,
        original_inventory - purchase_quantity
    );
}

#[test]
fn test_complete_shopping_flow_out_of_stock_handling() {
    // Simulate a user trying to purchase an out-of-stock item
    let service = get_test_product_service();

    // Step 1: User finds a product they want
    let filter = ProductFilter {
        name_contains: Some("Keyboard".to_string()),
        ..Default::default()
    };
    let products = service.filter(&filter);
    assert_eq!(products.len(), 1);

    let keyboard = &products[0];
    assert_eq!(keyboard.name, "Keyboard");

    // Step 2: Check if product is in stock
    assert_eq!(
        keyboard.inventory_count, 0,
        "Keyboard should be out of stock"
    );

    // Step 3: User decides to check other options (filters in-stock items)
    let in_stock_filter = ProductFilter {
        in_stock: Some(true),
        ..Default::default()
    };
    let available_products = service.filter(&in_stock_filter);

    // Step 4: Verify user has alternative options
    assert!(!available_products.is_empty());
    assert!(!available_products.iter().any(|p| p.name == "Keyboard"));
}

#[test]
fn test_complete_shopping_flow_multiple_items() {
    // Simulate a user purchasing multiple different items
    let service = get_test_product_service();

    // Step 1: User browses products
    let all_products = service.get_all();
    let initial_total = all_products.len();

    // Step 2: User selects multiple items (Mouse and Monitor)
    let mouse = service.get_by_id(2).unwrap();
    let monitor = service.get_by_id(4).unwrap();

    assert!(mouse.inventory_count >= 3);
    assert!(monitor.inventory_count >= 1);

    // Step 3: User "purchases" items
    let _ = service.update_inventory(mouse.id, mouse.inventory_count - 3);
    let _ = service.update_inventory(monitor.id, monitor.inventory_count - 1);

    // Step 4: Verify inventory updates
    let updated_mouse = service.get_by_id(2).unwrap();
    let updated_monitor = service.get_by_id(4).unwrap();

    assert_eq!(updated_mouse.inventory_count, mouse.inventory_count - 3);
    assert_eq!(updated_monitor.inventory_count, monitor.inventory_count - 1);

    // Step 5: Verify catalog integrity
    let final_products = service.get_all();
    assert_eq!(final_products.len(), initial_total);
}

// ============================================================================
// Admin/Inventory Management Flow Tests
// ============================================================================

#[test]
fn test_admin_flow_add_new_products() {
    // Simulate an admin adding new products to the catalog
    let service = get_test_product_service();

    let initial_count = service.get_all().len();

    // Step 1: Admin adds new product category (Headphones)
    let headphones = service.create(NewProduct {
        name: "Wireless Headphones".to_string(),
        description: "Noise-canceling headphones".to_string(),
        price: Decimal::from_str("199.99").unwrap(),
        inventory_count: 25,
    });

    // Step 2: Admin adds another variant
    let earbuds = service.create(NewProduct {
        name: "Wireless Earbuds".to_string(),
        description: "Compact earbuds".to_string(),
        price: Decimal::from_str("89.99").unwrap(),
        inventory_count: 50,
    });

    // Step 3: Verify products were added
    assert_eq!(service.get_all().len(), initial_count + 2);

    // Step 4: Verify products are searchable
    let audio_filter = ProductFilter {
        name_contains: Some("Wireless".to_string()),
        ..Default::default()
    };
    let audio_products = service.filter(&audio_filter);
    assert!(audio_products.iter().any(|p| p.id == headphones.id));
    assert!(audio_products.iter().any(|p| p.id == earbuds.id));
}

#[test]
fn test_admin_flow_restock_inventory() {
    // Simulate an admin restocking out-of-stock items
    let service = get_test_product_service();

    // Step 1: Find out-of-stock items
    let out_of_stock_filter = ProductFilter {
        in_stock: Some(false),
        ..Default::default()
    };
    let out_of_stock = service.filter(&out_of_stock_filter);

    assert_eq!(out_of_stock.len(), 1);
    let keyboard = &out_of_stock[0];
    assert_eq!(keyboard.inventory_count, 0);

    // Step 2: Admin restocks the item
    let restock_amount = 30;
    let result = service.update_inventory(keyboard.id, restock_amount);
    assert!(result.is_some());

    // Step 3: Verify item is now in stock
    let in_stock_filter = ProductFilter {
        in_stock: Some(true),
        ..Default::default()
    };
    let in_stock = service.filter(&in_stock_filter);

    assert!(in_stock.iter().any(|p| p.id == keyboard.id));

    // Step 4: Verify the exact inventory count
    let updated_keyboard = service.get_by_id(keyboard.id).unwrap();
    assert_eq!(updated_keyboard.inventory_count, restock_amount);
}

#[test]
fn test_admin_flow_bulk_inventory_update() {
    // Simulate an admin updating inventory for multiple products
    let service = get_test_product_service();

    let products = service.get_all();
    let updates = vec![
        (products[0].id, 100),
        (products[1].id, 75),
        (products[2].id, 50),
        (products[3].id, 25),
    ];

    // Step 1: Admin performs bulk update
    for (id, new_count) in &updates {
        let result = service.update_inventory(*id, *new_count);
        assert!(result.is_some(), "Update for product {id} should succeed");
    }

    // Step 2: Verify all updates
    for (id, expected_count) in updates {
        let product = service.get_by_id(id).unwrap();
        assert_eq!(product.inventory_count, expected_count);
    }
}

// ============================================================================
// Customer Search and Discovery Flow Tests
// ============================================================================

#[test]
fn test_customer_flow_price_comparison() {
    // Simulate a customer comparing products by price
    let service = get_test_product_service();

    // Step 1: Get all products
    let _all_products = service.get_all();

    // Step 2: Customer looks for budget options (under $50)
    let budget_filter = ProductFilter {
        max_price: Some(Decimal::from_str("50.00").unwrap()),
        in_stock: Some(true),
        ..Default::default()
    };
    let budget_options = service.filter(&budget_filter);

    assert_eq!(budget_options.len(), 1); // Only Mouse
    assert_eq!(budget_options[0].name, "Mouse");

    // Step 3: Customer looks for premium options (over $400)
    let premium_filter = ProductFilter {
        min_price: Some(Decimal::from_str("400.00").unwrap()),
        in_stock: Some(true),
        ..Default::default()
    };
    let premium_options = service.filter(&premium_filter);

    assert_eq!(premium_options.len(), 2); // Laptop and Monitor
    assert!(premium_options
        .iter()
        .all(|p| p.price >= Decimal::from_str("400.00").unwrap()));
}

#[test]
fn test_customer_flow_category_browse() {
    // Simulate a customer browsing specific product categories
    let service = get_test_product_service();

    // Step 1: Customer searches for input devices (containing "Mouse" or "Keyboard")
    let keyboard_filter = ProductFilter {
        name_contains: Some("board".to_string()),
        ..Default::default()
    };
    let keyboards = service.filter(&keyboard_filter);
    assert_eq!(keyboards.len(), 1);

    let mouse_filter = ProductFilter {
        name_contains: Some("Mouse".to_string()),
        ..Default::default()
    };
    let mice = service.filter(&mouse_filter);
    assert_eq!(mice.len(), 1);

    // Step 2: Customer filters for available input devices
    let available_input_filter = ProductFilter {
        name_contains: Some("o".to_string()), // Matches Mouse, Keyboard, Monitor
        in_stock: Some(true),
        ..Default::default()
    };
    let available = service.filter(&available_input_filter);

    // Should include Mouse and Monitor but not Keyboard
    assert!(available.iter().any(|p| p.name == "Mouse"));
    assert!(available.iter().any(|p| p.name == "Monitor"));
    assert!(!available.iter().any(|p| p.name == "Keyboard"));
}

// ============================================================================
// Edge Case Integration Tests
// ============================================================================

#[test]
fn test_integration_concurrent_service_access() {
    // Test that cloned services share state correctly
    let service1 = get_test_product_service();
    let service2 = service1.clone();

    // Step 1: Add product via service1
    let new_product = service1.create(NewProduct {
        name: "Shared Product".to_string(),
        description: "Test concurrent access".to_string(),
        price: Decimal::from_str("50.00").unwrap(),
        inventory_count: 10,
    });

    // Step 2: Retrieve via service2
    let retrieved = service2.get_by_id(new_product.id);
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().name, "Shared Product");

    // Step 3: Update via service2
    let _ = service2.update_inventory(new_product.id, 20);

    // Step 4: Verify via service1
    let updated = service1.get_by_id(new_product.id).unwrap();
    assert_eq!(updated.inventory_count, 20);
}

#[test]
fn test_integration_empty_to_full_catalog() {
    // Test building a catalog from scratch
    let service = cto_parallel_test::catalog::ProductService::new();

    // Verify empty
    assert_eq!(service.get_all().len(), 0);

    // Add products one by one
    let categories = vec![("Electronics", 5), ("Accessories", 3), ("Software", 2)];

    for (category, count) in categories {
        for i in 0..count {
            let _ = service.create(NewProduct {
                name: format!("{category} Product {i}"),
                description: format!("Description for {category}"),
                price: Decimal::from_str("99.99").unwrap(),
                inventory_count: 10,
            });
        }
    }

    // Verify catalog size
    assert_eq!(service.get_all().len(), 10);

    // Verify filtering works with new catalog
    let electronics_filter = ProductFilter {
        name_contains: Some("Electronics".to_string()),
        ..Default::default()
    };
    let electronics = service.filter(&electronics_filter);
    assert_eq!(electronics.len(), 5);
}

#[test]
fn test_integration_sequential_operations() {
    // Test a sequence of operations to ensure state consistency
    let service = get_test_product_service();

    // Operation 1: Create product
    let product = service.create(NewProduct {
        name: "Sequential Test".to_string(),
        description: "Testing sequential operations".to_string(),
        price: Decimal::from_str("100.00").unwrap(),
        inventory_count: 50,
    });

    let product_id = product.id;

    // Operation 2: Retrieve product
    let retrieved = service.get_by_id(product_id).unwrap();
    assert_eq!(retrieved.inventory_count, 50);

    // Operation 3: Update inventory
    let _ = service.update_inventory(product_id, 40);

    // Operation 4: Verify update
    let after_update = service.get_by_id(product_id).unwrap();
    assert_eq!(after_update.inventory_count, 40);

    // Operation 5: Filter to find product
    let filter = ProductFilter {
        name_contains: Some("Sequential".to_string()),
        ..Default::default()
    };
    let found = service.filter(&filter);
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].inventory_count, 40);
}

// ============================================================================
// Health Check Integration Test
// ============================================================================

#[test]
fn test_service_health_check() {
    // Verify the service is functioning correctly
    let service = get_test_product_service();

    // Basic functionality checks
    assert!(
        !service.get_all().is_empty(),
        "Service should have products"
    );

    // CRUD operations work
    let product = service.create(NewProduct {
        name: "Health Check".to_string(),
        description: "System verification".to_string(),
        price: Decimal::from_str("1.00").unwrap(),
        inventory_count: 1,
    });

    assert!(
        service.get_by_id(product.id).is_some(),
        "Created product should be retrievable"
    );

    // Filtering works
    let filter = ProductFilter {
        name_contains: Some("Health".to_string()),
        ..Default::default()
    };
    let found = service.filter(&filter);
    assert_eq!(found.len(), 1, "Filtering should work");

    // Inventory updates work
    let result = service.update_inventory(product.id, 100);
    assert!(result.is_some(), "Inventory updates should work");
}

// API integration tests for product catalog
// Tests product CRUD operations, filtering, and error handling

mod common;

use ecommerce_catalog::catalog::{NewProduct, ProductFilter, ProductService};
use rust_decimal_macros::dec;

#[test]
fn test_create_product() {
    let service = ProductService::new();
    let new_product = common::create_sample_product();

    let product = service.create(new_product);

    assert_eq!(product.id, 1);
    assert_eq!(product.name, "Test Product");
    assert_eq!(product.price, dec!(99.99));
    assert_eq!(product.inventory_count, 10);
}

#[test]
fn test_create_multiple_products_auto_increment_ids() {
    let service = ProductService::new();

    let product1 = service.create(NewProduct {
        name: "Product 1".to_string(),
        description: "First product".to_string(),
        price: dec!(10.00),
        inventory_count: 5,
    });

    let product2 = service.create(NewProduct {
        name: "Product 2".to_string(),
        description: "Second product".to_string(),
        price: dec!(20.00),
        inventory_count: 10,
    });

    let product3 = service.create(NewProduct {
        name: "Product 3".to_string(),
        description: "Third product".to_string(),
        price: dec!(30.00),
        inventory_count: 15,
    });

    assert_eq!(product1.id, 1);
    assert_eq!(product2.id, 2);
    assert_eq!(product3.id, 3);
}

#[test]
fn test_get_all_products() {
    let service = common::create_test_product_service();

    let products = service.get_all();

    assert_eq!(products.len(), 5);
    assert_eq!(products[0].name, "Laptop");
    assert_eq!(products[1].name, "Wireless Mouse");
    assert_eq!(products[2].name, "Mechanical Keyboard");
    assert_eq!(products[3].name, "4K Monitor");
    assert_eq!(products[4].name, "USB-C Hub");
}

#[test]
fn test_get_all_products_empty_catalog() {
    let service = ProductService::new();

    let products = service.get_all();

    assert_eq!(products.len(), 0);
}

#[test]
fn test_get_product_by_id() {
    let service = common::create_test_product_service();

    let product = service.get_by_id(1);

    assert!(product.is_some());
    let product = product.unwrap();
    assert_eq!(product.id, 1);
    assert_eq!(product.name, "Laptop");
    assert_eq!(product.price, dec!(1299.99));
}

#[test]
fn test_get_product_by_id_not_found() {
    let service = common::create_test_product_service();

    let product = service.get_by_id(999);

    assert!(product.is_none());
}

#[test]
fn test_get_product_by_id_zero() {
    let service = common::create_test_product_service();

    let product = service.get_by_id(0);

    assert!(product.is_none());
}

#[test]
fn test_get_product_by_id_negative() {
    let service = common::create_test_product_service();

    let product = service.get_by_id(-1);

    assert!(product.is_none());
}

#[test]
fn test_update_inventory() {
    let service = common::create_test_product_service();

    // Update existing product
    let updated = service.update_inventory(1, 100);

    assert!(updated.is_some());
    let updated_product = updated.unwrap();
    assert_eq!(updated_product.id, 1);
    assert_eq!(updated_product.inventory_count, 100);

    // Verify the change persisted
    let product = service.get_by_id(1).unwrap();
    assert_eq!(product.inventory_count, 100);
}

#[test]
fn test_update_inventory_to_zero() {
    let service = common::create_test_product_service();

    let updated = service.update_inventory(1, 0);

    assert!(updated.is_some());
    assert_eq!(updated.unwrap().inventory_count, 0);
}

#[test]
fn test_update_inventory_negative() {
    let service = common::create_test_product_service();

    let updated = service.update_inventory(1, -5);

    assert!(updated.is_some());
    assert_eq!(updated.unwrap().inventory_count, -5);
}

#[test]
fn test_update_inventory_not_found() {
    let service = common::create_test_product_service();

    let updated = service.update_inventory(999, 100);

    assert!(updated.is_none());
}

#[test]
fn test_delete_product() {
    let service = common::create_test_product_service();

    // Delete existing product
    let deleted = service.delete(1);

    assert!(deleted);

    // Verify it's gone
    let product = service.get_by_id(1);
    assert!(product.is_none());

    // Verify other products still exist
    let remaining = service.get_all();
    assert_eq!(remaining.len(), 4);
}

#[test]
fn test_delete_product_not_found() {
    let service = common::create_test_product_service();

    let deleted = service.delete(999);

    assert!(!deleted);
}

#[test]
fn test_delete_product_twice() {
    let service = common::create_test_product_service();

    // First delete succeeds
    assert!(service.delete(1));

    // Second delete fails
    assert!(!service.delete(1));
}

#[test]
fn test_filter_by_name() {
    let service = common::create_test_product_service();

    let filter = ProductFilter::new().with_name("Mouse".to_string());
    let results = service.filter(&filter);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "Wireless Mouse");
}

#[test]
fn test_filter_by_name_case_insensitive() {
    let service = common::create_test_product_service();

    let filter = ProductFilter::new().with_name("LAPTOP".to_string());
    let results = service.filter(&filter);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "Laptop");
}

#[test]
fn test_filter_by_name_partial_match() {
    let service = common::create_test_product_service();

    let filter = ProductFilter::new().with_name("key".to_string());
    let results = service.filter(&filter);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "Mechanical Keyboard");
}

#[test]
fn test_filter_by_name_no_matches() {
    let service = common::create_test_product_service();

    let filter = ProductFilter::new().with_name("NonexistentProduct".to_string());
    let results = service.filter(&filter);

    assert_eq!(results.len(), 0);
}

#[test]
fn test_filter_by_min_price() {
    let service = common::create_test_product_service();

    let filter = ProductFilter::new().with_min_price(dec!(100.00));
    let results = service.filter(&filter);

    // Should return Laptop (1299.99), Mechanical Keyboard (149.99), and 4K Monitor (599.99)
    assert_eq!(results.len(), 3);
    for product in results {
        assert!(product.price >= dec!(100.00));
    }
}

#[test]
fn test_filter_by_max_price() {
    let service = common::create_test_product_service();

    let filter = ProductFilter::new().with_max_price(dec!(50.00));
    let results = service.filter(&filter);

    // Should return Wireless Mouse (29.99) and USB-C Hub (45.99)
    assert_eq!(results.len(), 2);
    for product in results {
        assert!(product.price <= dec!(50.00));
    }
}

#[test]
fn test_filter_by_price_range() {
    let service = common::create_test_product_service();

    let filter = ProductFilter::new()
        .with_min_price(dec!(30.00))
        .with_max_price(dec!(200.00));
    let results = service.filter(&filter);

    // Should return Mechanical Keyboard (149.99) and USB-C Hub (45.99)
    assert_eq!(results.len(), 2);
    for product in results {
        assert!(product.price >= dec!(30.00) && product.price <= dec!(200.00));
    }
}

#[test]
fn test_filter_by_price_exact() {
    let service = common::create_test_product_service();

    let filter = ProductFilter::new()
        .with_min_price(dec!(29.99))
        .with_max_price(dec!(29.99));
    let results = service.filter(&filter);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "Wireless Mouse");
}

#[test]
fn test_filter_by_in_stock() {
    let service = common::create_test_product_service();

    let filter = ProductFilter::new().with_in_stock(true);
    let results = service.filter(&filter);

    // Should return all except 4K Monitor (which has 0 inventory)
    assert_eq!(results.len(), 4);
    for product in results {
        assert!(product.inventory_count > 0);
    }
}

#[test]
fn test_filter_by_out_of_stock() {
    let service = common::create_test_product_service();

    let filter = ProductFilter::new().with_in_stock(false);
    let results = service.filter(&filter);

    // Should return only 4K Monitor
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "4K Monitor");
    assert_eq!(results[0].inventory_count, 0);
}

#[test]
fn test_filter_combined() {
    let service = common::create_test_product_service();

    let filter = ProductFilter::new()
        .with_name("key".to_string())
        .with_min_price(dec!(100.00))
        .with_in_stock(true);
    let results = service.filter(&filter);

    // Should return only Mechanical Keyboard
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "Mechanical Keyboard");
}

#[test]
fn test_filter_combined_no_matches() {
    let service = common::create_test_product_service();

    let filter = ProductFilter::new()
        .with_name("Monitor".to_string())
        .with_in_stock(true); // Monitor is out of stock
    let results = service.filter(&filter);

    assert_eq!(results.len(), 0);
}

#[test]
fn test_filter_empty_returns_all() {
    let service = common::create_test_product_service();

    let filter = ProductFilter::new();
    let results = service.filter(&filter);

    assert_eq!(results.len(), 5);
}

#[test]
fn test_product_decimal_precision() {
    let service = ProductService::new();

    let product = service.create(NewProduct {
        name: "Precise Product".to_string(),
        description: "Test decimal precision".to_string(),
        price: dec!(19.999),
        inventory_count: 1,
    });

    assert_eq!(product.price, dec!(19.999));

    let fetched = service.get_by_id(product.id).unwrap();
    assert_eq!(fetched.price, dec!(19.999));
}

#[test]
fn test_product_zero_price() {
    let service = ProductService::new();

    let product = service.create(NewProduct {
        name: "Free Product".to_string(),
        description: "Zero price product".to_string(),
        price: dec!(0.00),
        inventory_count: 100,
    });

    assert_eq!(product.price, dec!(0.00));
}

#[test]
fn test_product_large_inventory() {
    let service = ProductService::new();

    let product = service.create(NewProduct {
        name: "Bulk Product".to_string(),
        description: "Large inventory".to_string(),
        price: dec!(1.00),
        inventory_count: 1_000_000,
    });

    assert_eq!(product.inventory_count, 1_000_000);
}

#[test]
fn test_product_empty_description() {
    let service = ProductService::new();

    let product = service.create(NewProduct {
        name: "No Description".to_string(),
        description: String::new(),
        price: dec!(10.00),
        inventory_count: 5,
    });

    assert_eq!(product.description, "");
}

#[test]
fn test_product_long_name_and_description() {
    let service = ProductService::new();

    let long_name = "A".repeat(500);
    let long_description = "B".repeat(2000);

    let product = service.create(NewProduct {
        name: long_name.clone(),
        description: long_description.clone(),
        price: dec!(100.00),
        inventory_count: 10,
    });

    assert_eq!(product.name, long_name);
    assert_eq!(product.description, long_description);
}

#[test]
fn test_product_unicode_name_and_description() {
    let service = ProductService::new();

    let product = service.create(NewProduct {
        name: "日本語の商品名 🛒".to_string(),
        description: "製品の説明 with émojis 🎉".to_string(),
        price: dec!(50.00),
        inventory_count: 20,
    });

    assert_eq!(product.name, "日本語の商品名 🛒");
    assert_eq!(product.description, "製品の説明 with émojis 🎉");
}

#[test]
fn test_concurrent_product_operations() {
    use std::sync::Arc;
    use std::thread;

    let service = Arc::new(ProductService::new());
    let mut handles = vec![];

    // Spawn multiple threads creating products
    for i in 0..10 {
        let service_clone = Arc::clone(&service);
        let handle = thread::spawn(move || {
            service_clone.create(NewProduct {
                name: format!("Product {i}"),
                description: format!("Description {i}"),
                price: dec!(10.00),
                inventory_count: i,
            })
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().expect("Thread panicked");
    }

    // Verify all products were created
    let products = service.get_all();
    assert_eq!(products.len(), 10);
}

#[test]
fn test_service_clone_shares_data() {
    let service = ProductService::new();
    let service_clone = service.clone();

    // Create product in original service
    let product = service.create(NewProduct {
        name: "Original Product".to_string(),
        description: "Created in original".to_string(),
        price: dec!(50.00),
        inventory_count: 10,
    });

    // Should be visible in clone
    let fetched = service_clone.get_by_id(product.id);
    assert!(fetched.is_some());
    assert_eq!(fetched.unwrap().name, "Original Product");

    // Modifications in clone should be visible in original
    let _ = service_clone.update_inventory(product.id, 99);
    let updated_in_original = service.get_by_id(product.id).unwrap();
    assert_eq!(updated_in_original.inventory_count, 99);
}

#[test]
fn test_filter_builder_pattern() {
    let service = common::create_test_product_service();

    // Test building filter with chained methods
    let filter = ProductFilter::new()
        .with_name("Laptop".to_string())
        .with_min_price(dec!(1000.00))
        .with_max_price(dec!(2000.00))
        .with_in_stock(true);

    let results = service.filter(&filter);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "Laptop");
}

#[test]
fn test_default_product_filter() {
    let filter1 = ProductFilter::new();
    let filter2 = ProductFilter::default();

    // Both should be equivalent
    assert!(filter1.name_contains.is_none());
    assert!(filter1.min_price.is_none());
    assert!(filter1.max_price.is_none());
    assert!(filter1.in_stock.is_none());

    assert!(filter2.name_contains.is_none());
    assert!(filter2.min_price.is_none());
    assert!(filter2.max_price.is_none());
    assert!(filter2.in_stock.is_none());
}

#[test]
fn test_product_serialization_and_deserialization() {
    let service = ProductService::new();
    let product = service.create(common::create_sample_product());

    // Serialize to JSON
    let json = serde_json::to_string(&product).expect("Failed to serialize");

    // Deserialize back
    let deserialized: ecommerce_catalog::catalog::Product =
        serde_json::from_str(&json).expect("Failed to deserialize");

    assert_eq!(product.id, deserialized.id);
    assert_eq!(product.name, deserialized.name);
    assert_eq!(product.description, deserialized.description);
    assert_eq!(product.price, deserialized.price);
    assert_eq!(product.inventory_count, deserialized.inventory_count);
}

#[test]
fn test_new_product_serialization() {
    let new_product = common::create_sample_product();

    let json = serde_json::to_string(&new_product).expect("Failed to serialize");
    let deserialized: NewProduct = serde_json::from_str(&json).expect("Failed to deserialize");

    assert_eq!(new_product.name, deserialized.name);
    assert_eq!(new_product.description, deserialized.description);
    assert_eq!(new_product.price, deserialized.price);
    assert_eq!(new_product.inventory_count, deserialized.inventory_count);
}

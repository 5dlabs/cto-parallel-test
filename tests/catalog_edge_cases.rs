use cto_parallel_test::catalog::{NewProduct, ProductFilter, ProductService};
use rust_decimal::Decimal;

fn build_product(name: &str, description: &str, price: Decimal, inventory: i32) -> NewProduct {
    NewProduct {
        name: name.to_string(),
        description: description.to_string(),
        price,
        inventory_count: inventory,
    }
}

#[test]
fn test_update_inventory_nonexistent_returns_none() {
    let svc = ProductService::new();
    assert!(svc.update_inventory(9999, 10).is_none());
}

#[test]
fn test_update_inventory_allows_negative_counts() {
    let svc = ProductService::new();
    let product = svc.create(build_product(
        "Test",
        "Negative inventory",
        Decimal::new(100, 2),
        10,
    ));

    let updated = svc
        .update_inventory(product.id, -5)
        .expect("update inventory should succeed");
    assert_eq!(-5, updated.inventory_count);
}

#[test]
fn test_filter_price_bounds() {
    let svc = ProductService::new();

    let _ = svc.create(build_product(
        "Budget",
        "Affordable option",
        Decimal::new(50, 2),
        5,
    ));
    let _ = svc.create(build_product(
        "Midrange",
        "Balanced choice",
        Decimal::new(1500, 2),
        8,
    ));
    let _ = svc.create(build_product(
        "Premium",
        "High-end product",
        Decimal::new(5000, 2),
        3,
    ));

    let filter = ProductFilter {
        min_price: Some(Decimal::new(100, 2)),
        max_price: Some(Decimal::new(3000, 2)),
        ..ProductFilter::default()
    };

    let results = svc.filter(filter);
    assert_eq!(1, results.len());
    assert_eq!("Midrange", results[0].name);
}

#[test]
fn test_filter_combined_criteria() {
    let svc = ProductService::new();

    let _ = svc.create(build_product(
        "Apple Juice",
        "Refreshing drink",
        Decimal::new(299, 2),
        15,
    ));
    let _ = svc.create(build_product(
        "Apple Pie",
        "Dessert",
        Decimal::new(599, 2),
        5,
    ));
    let _ = svc.create(build_product(
        "Orange Juice",
        "Citrus",
        Decimal::new(250, 2),
        20,
    ));

    let filter = ProductFilter {
        name_contains: Some("apple".to_string()),
        min_price: Some(Decimal::new(200, 2)),
        max_price: Some(Decimal::new(400, 2)),
        in_stock: Some(true),
    };

    let results = svc.filter(filter);
    assert_eq!(1, results.len());
    assert_eq!("Apple Juice", results[0].name);
}

#[test]
fn test_delete_nonexistent() {
    let svc = ProductService::new();
    assert!(!svc.delete(9999));
}

#[test]
fn test_id_auto_increment_starts_at_one() {
    let svc = ProductService::new();
    let product = svc.create(build_product(
        "First",
        "First product",
        Decimal::new(100, 2),
        10,
    ));

    assert_eq!(1, product.id);
}

#[test]
fn test_id_auto_increment_sequential() {
    let svc = ProductService::new();

    let p1 = svc.create(build_product(
        "First",
        "First product",
        Decimal::new(100, 2),
        10,
    ));
    let p2 = svc.create(build_product(
        "Second",
        "Second product",
        Decimal::new(200, 2),
        20,
    ));
    let p3 = svc.create(build_product(
        "Third",
        "Third product",
        Decimal::new(300, 2),
        30,
    ));

    assert_eq!(1, p1.id);
    assert_eq!(2, p2.id);
    assert_eq!(3, p3.id);
}

#[test]
fn test_decimal_precision_maintained() {
    let svc = ProductService::new();

    let product = svc.create(build_product(
        "Precise Item",
        "High precision",
        Decimal::new(12345, 2), // 123.45
        10,
    ));

    assert_eq!(Decimal::new(12345, 2), product.price);

    let filter = ProductFilter {
        min_price: Some(Decimal::new(12345, 2)),
        max_price: Some(Decimal::new(12345, 2)),
        ..ProductFilter::default()
    };

    let results = svc.filter(filter);
    assert_eq!(1, results.len());
}

#[test]
fn test_case_insensitive_name_filter() {
    let svc = ProductService::new();

    let _ = svc.create(build_product("Apple", "Fruit", Decimal::new(100, 2), 10));

    let filter = ProductFilter {
        name_contains: Some("APPLE".to_string()),
        ..ProductFilter::default()
    };
    assert_eq!(1, svc.filter(filter).len());

    let filter = ProductFilter {
        name_contains: Some("apple".to_string()),
        ..ProductFilter::default()
    };
    assert_eq!(1, svc.filter(filter).len());

    let filter = ProductFilter {
        name_contains: Some("ApPlE".to_string()),
        ..ProductFilter::default()
    };
    assert_eq!(1, svc.filter(filter).len());
}

#[test]
fn test_empty_filter_returns_all() {
    let svc = ProductService::new();

    let _ = svc.create(build_product(
        "Product 1",
        "First",
        Decimal::new(100, 2),
        10,
    ));
    let _ = svc.create(build_product(
        "Product 2",
        "Second",
        Decimal::new(200, 2),
        20,
    ));

    let results = svc.filter(ProductFilter::default());
    assert_eq!(2, results.len());
}

#[test]
fn test_clone_does_not_expose_internal_state() {
    let svc = ProductService::new();

    let product = svc.create(build_product(
        "Test",
        "Clone safety",
        Decimal::new(100, 2),
        10,
    ));

    let mut all = svc.get_all();
    all[0].inventory_count = 999;

    let retrieved = svc.get_by_id(product.id).expect("get product");
    assert_eq!(10, retrieved.inventory_count);
}

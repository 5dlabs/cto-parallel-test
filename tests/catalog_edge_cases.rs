use cto_parallel_test::catalog::models::{MAX_NAME_LEN, MAX_STOCK};
use cto_parallel_test::catalog::{NewProduct, ProductFilter, ProductService};
use rust_decimal::Decimal;

#[test]
fn test_validation_empty_name() {
    let svc = ProductService::new();
    let result = svc.create(NewProduct {
        name: String::new(),
        price: Decimal::new(100, 2),
        stock: 10,
    });
    assert!(result.is_err());
}

#[test]
fn test_validation_negative_price() {
    let svc = ProductService::new();
    let result = svc.create(NewProduct {
        name: "Test".to_string(),
        price: Decimal::new(-100, 2),
        stock: 10,
    });
    assert!(result.is_err());
}

#[test]
fn test_validation_negative_stock() {
    let svc = ProductService::new();
    let result = svc.create(NewProduct {
        name: "Test".to_string(),
        price: Decimal::new(100, 2),
        stock: -1,
    });
    assert!(result.is_err());
}

#[test]
fn test_update_inventory_negative_fails() {
    let svc = ProductService::new();
    let product = svc
        .create(NewProduct {
            name: "Test".to_string(),
            price: Decimal::new(100, 2),
            stock: 10,
        })
        .expect("create product");

    let result = svc.update_inventory(product.id, -1);
    assert!(result.is_err());
}

#[test]
fn test_update_inventory_not_found() {
    let svc = ProductService::new();
    let result = svc.update_inventory(9999, 10);
    assert!(result.is_err());
}

#[test]
fn test_validation_name_too_long() {
    let svc = ProductService::new();
    // Construct a name that exceeds MAX_NAME_LEN by 1 character
    let too_long = "a".repeat(MAX_NAME_LEN + 1);
    let result = svc.create(NewProduct {
        name: too_long,
        price: Decimal::new(100, 2),
        stock: 10,
    });
    assert!(result.is_err());
}

#[test]
fn test_update_inventory_above_max_fails() {
    let svc = ProductService::new();
    let product = svc
        .create(NewProduct {
            name: "Test".to_string(),
            price: Decimal::new(100, 2),
            stock: 10,
        })
        .expect("create product");

    let result = svc.update_inventory(product.id, MAX_STOCK + 1);
    assert!(result.is_err());
}

#[test]
fn test_filter_min_stock() {
    let svc = ProductService::new();

    svc.create(NewProduct {
        name: "Low Stock".to_string(),
        price: Decimal::new(100, 2),
        stock: 2,
    })
    .expect("create");

    svc.create(NewProduct {
        name: "High Stock".to_string(),
        price: Decimal::new(100, 2),
        stock: 20,
    })
    .expect("create");

    let filter = ProductFilter {
        min_stock: Some(10),
        ..ProductFilter::default()
    };

    let results = svc.filter(&filter);
    assert_eq!(1, results.len());
    assert_eq!("High Stock", results[0].name);
}

#[test]
fn test_filter_max_stock() {
    let svc = ProductService::new();

    svc.create(NewProduct {
        name: "Low Stock".to_string(),
        price: Decimal::new(100, 2),
        stock: 2,
    })
    .expect("create");

    svc.create(NewProduct {
        name: "High Stock".to_string(),
        price: Decimal::new(100, 2),
        stock: 20,
    })
    .expect("create");

    let filter = ProductFilter {
        max_stock: Some(10),
        ..ProductFilter::default()
    };

    let results = svc.filter(&filter);
    assert_eq!(1, results.len());
    assert_eq!("Low Stock", results[0].name);
}

#[test]
fn test_filter_combined_criteria() {
    let svc = ProductService::new();

    svc.create(NewProduct {
        name: "Apple Juice".to_string(),
        price: Decimal::new(299, 2),
        stock: 15,
    })
    .expect("create");

    svc.create(NewProduct {
        name: "Apple Pie".to_string(),
        price: Decimal::new(599, 2),
        stock: 5,
    })
    .expect("create");

    svc.create(NewProduct {
        name: "Orange Juice".to_string(),
        price: Decimal::new(250, 2),
        stock: 20,
    })
    .expect("create");

    let filter = ProductFilter {
        name_contains: Some("apple".to_string()),
        min_price: Some(Decimal::new(200, 2)),
        max_price: Some(Decimal::new(400, 2)),
        min_stock: Some(10),
        ..ProductFilter::default()
    };

    let results = svc.filter(&filter);
    assert_eq!(1, results.len());
    assert_eq!("Apple Juice", results[0].name);
}

#[test]
fn test_delete_nonexistent() {
    let svc = ProductService::new();
    let result = svc.delete(9999);
    assert!(!result);
}

#[test]
fn test_id_auto_increment_starts_at_one() {
    let svc = ProductService::new();
    let product = svc
        .create(NewProduct {
            name: "First".to_string(),
            price: Decimal::new(100, 2),
            stock: 10,
        })
        .expect("create");

    assert_eq!(1, product.id);
}

#[test]
fn test_id_auto_increment_sequential() {
    let svc = ProductService::new();

    let p1 = svc
        .create(NewProduct {
            name: "First".to_string(),
            price: Decimal::new(100, 2),
            stock: 10,
        })
        .expect("create");

    let p2 = svc
        .create(NewProduct {
            name: "Second".to_string(),
            price: Decimal::new(200, 2),
            stock: 20,
        })
        .expect("create");

    let p3 = svc
        .create(NewProduct {
            name: "Third".to_string(),
            price: Decimal::new(300, 2),
            stock: 30,
        })
        .expect("create");

    assert_eq!(1, p1.id);
    assert_eq!(2, p2.id);
    assert_eq!(3, p3.id);
}

#[test]
fn test_decimal_precision_maintained() {
    let svc = ProductService::new();

    // Test various decimal values
    let product = svc
        .create(NewProduct {
            name: "Precise Item".to_string(),
            price: Decimal::new(12345, 2), // 123.45
            stock: 10,
        })
        .expect("create");

    assert_eq!(Decimal::new(12345, 2), product.price);

    // Test price comparison in filter
    let filter = ProductFilter {
        min_price: Some(Decimal::new(12345, 2)),
        max_price: Some(Decimal::new(12345, 2)),
        ..ProductFilter::default()
    };

    let results = svc.filter(&filter);
    assert_eq!(1, results.len());
}

#[test]
fn test_case_insensitive_name_filter() {
    let svc = ProductService::new();

    svc.create(NewProduct {
        name: "Apple".to_string(),
        price: Decimal::new(100, 2),
        stock: 10,
    })
    .expect("create");

    // Test uppercase search
    let filter = ProductFilter {
        name_contains: Some("APPLE".to_string()),
        ..ProductFilter::default()
    };
    let results = svc.filter(&filter);
    assert_eq!(1, results.len());

    // Test lowercase search
    let filter = ProductFilter {
        name_contains: Some("apple".to_string()),
        ..ProductFilter::default()
    };
    let results = svc.filter(&filter);
    assert_eq!(1, results.len());

    // Test mixed case search
    let filter = ProductFilter {
        name_contains: Some("ApPlE".to_string()),
        ..ProductFilter::default()
    };
    let results = svc.filter(&filter);
    assert_eq!(1, results.len());
}

#[test]
fn test_empty_filter_returns_all() {
    let svc = ProductService::new();

    svc.create(NewProduct {
        name: "Product 1".to_string(),
        price: Decimal::new(100, 2),
        stock: 10,
    })
    .expect("create");

    svc.create(NewProduct {
        name: "Product 2".to_string(),
        price: Decimal::new(200, 2),
        stock: 20,
    })
    .expect("create");

    let filter = ProductFilter::default();
    let results = svc.filter(&filter);
    assert_eq!(2, results.len());
}

#[test]
fn test_clone_does_not_expose_internal_state() {
    let svc = ProductService::new();

    let product = svc
        .create(NewProduct {
            name: "Test".to_string(),
            price: Decimal::new(100, 2),
            stock: 10,
        })
        .expect("create");

    // Get all products and modify the returned vector
    let mut all = svc.get_all();
    all[0].stock = 999;

    // Original product should be unchanged
    let retrieved = svc.get_by_id(product.id).expect("get product");
    assert_eq!(10, retrieved.stock);
}

use cto_parallel_test::catalog::{NewProduct, ProductFilter, ProductService};
use rust_decimal::Decimal;
use std::sync::Arc;
use std::thread;

#[test]
fn crud_and_filter_and_precision() {
    let svc = ProductService::new();

    // Create products
    let apple = svc.create(NewProduct {
        name: "Apple".to_string(),
        description: "Crisp and fresh".to_string(),
        price: Decimal::new(199, 2), // 1.99
        inventory_count: 10,
    });
    let banana = svc.create(NewProduct {
        name: "Banana".to_string(),
        description: "Sweet and ripe".to_string(),
        price: Decimal::new(99, 2), // 0.99
        inventory_count: 0,
    });

    assert!(apple.id != banana.id);
    assert_eq!(Decimal::new(199, 2), apple.price);
    assert_eq!(Decimal::new(99, 2), banana.price);

    // get_all and get_by_id
    let all = svc.get_all();
    assert_eq!(2, all.len());
    assert_eq!(Some(apple.clone()), svc.get_by_id(apple.id));
    assert_eq!(None, svc.get_by_id(9999));

    // update inventory
    let updated = svc
        .update_inventory(apple.id, 5)
        .expect("update apple stock");
    assert_eq!(5, updated.inventory_count);

    // filter by name
    let f = ProductFilter {
        name_contains: Some("app".into()),
        ..ProductFilter::default()
    };
    let results = svc.filter(f);
    assert_eq!(1, results.len());
    assert_eq!("Apple", results[0].name);

    // filter by price range and stock
    let f = ProductFilter {
        min_price: Some(Decimal::new(100, 2)),
        max_price: Some(Decimal::new(300, 2)),
        in_stock: Some(true),
        ..ProductFilter::default()
    };
    let results = svc.filter(f);
    assert_eq!(1, results.len());
    assert_eq!("Apple", results[0].name);

    // in_stock false should find banana only
    let f = ProductFilter {
        in_stock: Some(false),
        ..ProductFilter::default()
    };
    let results = svc.filter(f);
    assert_eq!(1, results.len());
    assert_eq!("Banana", results[0].name);

    // delete
    let deleted = svc.delete(banana.id);
    assert!(deleted);
    assert_eq!(1, svc.get_all().len());
}

#[test]
fn concurrency_create_and_update() {
    let svc = Arc::new(ProductService::new());
    let threads = 8;
    let per_thread = 25;

    // Spawn threads to create products concurrently
    let mut handles = Vec::new();
    for t in 0..threads {
        let svc_cloned = Arc::clone(&svc);
        handles.push(thread::spawn(move || {
            for i in 0..per_thread {
                let _ = svc_cloned.create(NewProduct {
                    name: format!("Item-{t}-{i}"),
                    description: "Bulk".to_string(),
                    price: Decimal::new(1234, 2),
                    inventory_count: 1,
                });
            }
        }));
    }
    for h in handles {
        h.join().expect("thread join");
    }

    let all = svc.get_all();
    assert_eq!(threads * per_thread, all.len());

    // Update a few inventories in parallel
    let ids: Vec<i32> = all.iter().take(10).map(|p| p.id).collect();
    let mut handles = Vec::new();
    for id in ids {
        let svc_cloned = Arc::clone(&svc);
        handles.push(thread::spawn(move || {
            let _ = svc_cloned
                .update_inventory(id, 7)
                .expect("update inventory");
        }));
    }
    for h in handles {
        h.join().expect("thread join");
    }

    let updated = svc.get_all();
    let sevens = updated.iter().filter(|p| p.inventory_count == 7).count();
    assert_eq!(10, sevens);
}

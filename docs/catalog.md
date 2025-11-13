# Catalog Module

This module provides a thread-safe, in-memory product catalog with CRUD operations, inventory tracking, and flexible filtering.

Key points:
- Thread-safe storage using `Arc<Mutex<Vec<Product>>>`
- Auto-incrementing product IDs
- Decimal precision for prices via `rust_decimal::Decimal`
- Filtering by name substring, price range, and in/out of stock

## Usage

Add the module to your crate root:

```
pub mod catalog;
```

Construct and use the service:

```
use cto_parallel_test::catalog::{NewProduct, ProductFilter, ProductService};
use rust_decimal::Decimal;

let svc = ProductService::new();
let created = svc.create(NewProduct {
    name: "Laptop".into(),
    description: "15\" ultrabook".into(),
    price: Decimal::new(99999, 2), // 999.99
    inventory_count: 10,
});

let found = svc.get_by_id(created.id);

let results = svc.filter(&ProductFilter {
    name_contains: Some("laptop".into()),
    ..Default::default()
});
```

## Concurrency & Safety

- All operations synchronize on internal `Mutex` guards.
- Secure defaults: mutex poisoning results in a panic (fail-closed) to avoid operating on potentially corrupted state.
- No unsafe code is used (`#![forbid(unsafe_code)]`).

## Testing

Run:

```
cargo test --workspace --all-features
```

The test suite covers CRUD operations, concurrent access, filtering, and decimal precision.

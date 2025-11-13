# Catalog Module

Thread-safe in-memory product catalog providing CRUD, inventory tracking, and flexible filtering with precise decimal pricing.

## Features
- Thread-safe storage (`Arc<Mutex<Vec<Product>>>`)
- Auto-incrementing integer IDs
- Decimal prices via `rust_decimal::Decimal`
- CRUD: create, list, get by id, delete
- Inventory updates with read-after-write consistency
- Filtering by name substring (case-insensitive), price min/max, and stock state

## Public API
- `catalog::ProductService` – main entry point
- `catalog::models::{Product, NewProduct, ProductFilter}`

## Usage
```rust
use cto_parallel_test::catalog::{ProductService, ProductFilter};
use cto_parallel_test::catalog::Product;
use cto_parallel_test::catalog::NewProduct;
use rust_decimal::Decimal;
use std::str::FromStr;

let svc = ProductService::new();
let p = svc.create(NewProduct {
    name: "Laptop".into(),
    description: "14\" ultrabook".into(),
    price: Decimal::from_str("999.99").unwrap(),
    inventory_count: 10,
});

let by_id = svc.get_by_id(p.id);
let all = svc.get_all();
let _ = svc.update_inventory(p.id, 12);

let results = svc.filter(&ProductFilter {
    name_contains: Some("laptop".into()),
    ..Default::default()
});
```

## Security & Quality
- No unsafe Rust (`#![forbid(unsafe_code)]`)
- All inputs are plain data; no serialization of untrusted input
- No hardcoded secrets or external IO in this module
- Verified with `cargo fmt`, `cargo clippy` (pedantic, deny warnings), and `cargo test`
- Fail-closed design: if a mutex is poisoned, operations panic with context rather than proceed in an unknown state

## Conventions
- Adheres to coding patterns in `coding-guidelines.md` (immutability by default, clear error contexts, small units)
- Follows PR and review process in `github-guidelines.md` (labels, descriptions, and CI checks)

## Notes
- This module has no external dependencies beyond `serde` and `rust_decimal`.
- Suitable as a foundation for higher-level API layers and persistence backends.

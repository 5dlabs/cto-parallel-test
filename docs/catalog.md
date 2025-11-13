# Product Catalog Module

This module provides a thread-safe, in-memory product catalog with CRUD, inventory management, and flexible filtering.

## Highlights

- Thread-safe storage via `Arc<Mutex<...>>`
- Auto-incrementing product IDs (monotonic, unique per process)
- Decimal-precise prices using `rust_decimal::Decimal`
- Filtering by name (case-insensitive contains), price range, and stock status
- No external dependencies beyond `serde` and `rust_decimal`

## Module Layout

- `src/catalog/models.rs`
  - `Product` — full product record
  - `NewProduct` — inputs for creation
  - `ProductFilter` — optional criteria for filtering
- `src/catalog/service.rs`
  - `ProductService` — thread-safe service exposing CRUD + filter APIs

## Usage

```rust
use cto_parallel_test::catalog::{NewProduct, ProductFilter, ProductService};
use rust_decimal::Decimal;
use std::str::FromStr;

let svc = ProductService::new();

let p = svc.create(NewProduct {
    name: "Laptop".into(),
    description: "14-inch ultrabook".into(),
    price: Decimal::from_str("999.99").unwrap(),
    inventory_count: 10,
});

let found = svc.get_by_id(p.id);
assert!(found.is_some());

let results = svc.filter(&ProductFilter {
    name_contains: Some("laptop".into()),
    ..Default::default()
});
assert!(!results.is_empty());
```

## Concurrency Model

- Internal state uses two independent locks:
  - `next_id: Mutex<i32>` for ID allocation (held briefly per create)
  - `products: Mutex<Vec<Product>>` for data access/mutation
- Lock acquisition is narrow to minimize contention and avoid deadlocks.
- On poisoned mutex, the service fails closed via panic to prevent serving inconsistent state.

## Security Notes

- No I/O, deserialization from untrusted inputs, or external commands.
- No secrets in code; all behavior is deterministic and in-memory.
- Panics are restricted to poisoned-lock scenarios (fail secure-by-default).

## Tests

Unit tests cover:
- CRUD operations and ID auto-increment
- Decimal precision invariants
- Filtering for all criteria and combined cases
- Concurrency under create and mixed read/write workloads


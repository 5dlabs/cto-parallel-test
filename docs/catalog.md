# Product Catalog Module

This module provides a thread-safe, in-memory product catalog with full CRUD operations, inventory management, and flexible filtering. It is dependency-light and designed as a foundational building block for higher-level API services.

## Highlights
- Thread-safe storage via `Arc<Mutex<Vec<Product>>>`
- Auto-incrementing product IDs
- Decimal-precise pricing using `rust_decimal::Decimal`
- CRUD: create, list, get by ID, delete
- Inventory updates with read-after-write semantics
- Filtering by name substring, price range, and stock status (AND semantics)

## Module Layout
- `src/catalog/models.rs` — Data models (`Product`, `NewProduct`, `ProductFilter`)
- `src/catalog/service.rs` — `ProductService` with thread-safe operations and tests
- `src/catalog/mod.rs` — Public exports
- `src/lib.rs` — Registers `pub mod catalog;`

## Usage

```rust
use cto_parallel_test::catalog::{NewProduct, ProductFilter, ProductService};
use rust_decimal::Decimal;

let service = ProductService::new();

// Create
let p = service.create(NewProduct {
    name: "Laptop".into(),
    description: "13-inch, 16GB RAM".into(),
    price: Decimal::from_str_exact("999.99").unwrap(),
    inventory_count: 10,
});

// Read
let all = service.get_all();
let one = service.get_by_id(p.id);

// Update inventory
let _ = service.update_inventory(p.id, 8);

// Filter
let filtered = service.filter(&ProductFilter {
    name_contains: Some("lap".into()),
    min_price: None,
    max_price: None,
    in_stock: Some(true),
});

// Delete
let _deleted = service.delete(p.id);
```

## Concurrency Guarantees
- Internal state protected with `Mutex`; operations fail closed if a lock is poisoned.
- ID allocation and product mutations do not hold multiple locks simultaneously, avoiding deadlocks.
- Comprehensive concurrency tests validate creation and mixed read/write patterns.

## Security Considerations
- No external I/O or OS command execution; no SQL, deserialization, or filesystem paths involved.
- No secrets stored; all data is in-process memory only.
- Fail-secure defaults: poisoned locks cause a controlled panic rather than returning inconsistent state.

## Testing
- Run the standard gates (see `coding-guidelines.md`):
  - `cargo fmt --all -- --check`
  - `cargo clippy --workspace --all-targets --all-features -- -D warnings -W clippy::pedantic`
  - `cargo test --workspace --all-features`

Unit tests cover CRUD operations, filtering, concurrency behavior, and decimal precision.


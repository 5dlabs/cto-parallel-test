# Catalog Module

This module provides a thread-safe, in-memory product catalog with CRUD operations, inventory management, and flexible filtering using precise decimal pricing.

- Storage: `Arc<Mutex<Vec<Product>>>`
- IDs: Auto-incrementing `i32`
- Price: `rust_decimal::Decimal` (no floating-point rounding errors)
- Filters: name contains, min/max price, in-stock toggle (AND logic)

## Modules
- `src/catalog/models.rs` — `Product`, `NewProduct`, `ProductFilter`
- `src/catalog/service.rs` — `ProductService` (CRUD, filter, inventory)
- `src/catalog/mod.rs` — Module exports
- `src/lib.rs` — Registers `pub mod catalog;`

## Usage
```rust
use cto_parallel_test::catalog::{NewProduct, ProductFilter, ProductService};
use rust_decimal::Decimal;

let svc = ProductService::new();
let p = svc.create(NewProduct {
    name: "Laptop".into(),
    description: "15\" display".into(),
    price: Decimal::new(129999, 2), // 1299.99
    inventory_count: 10,
});

let updated = svc.update_inventory(p.id, 8);
let results = svc.filter(&ProductFilter { name_contains: Some("laptop".into()), ..Default::default() });
```

## Testing
- Unit tests cover CRUD, filtering, decimal precision, and concurrency
- Run: `cargo test --workspace --all-features`

## Security Notes
- No external I/O, SQL, shell, or paths -> low risk surface
- No hardcoded secrets
- Uses safe synchronization primitives; no `unsafe`

```
$ cargo fmt --all -- --check
$ cargo clippy --workspace --all-targets --all-features -- -D warnings -W clippy::pedantic
$ cargo test --workspace --all-features
```

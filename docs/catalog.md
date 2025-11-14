# Product Catalog Module

This module provides a thread-safe, in-memory product catalog with:

- CRUD operations for `Product`
- Inventory updates with validation
- Flexible filtering by name, price, and stock
- Precise prices via `rust_decimal::Decimal`
- Auto-incrementing IDs managed internally by the service

## Security & Quality

- No hardcoded secrets or external IO
- Input validation and sanitization for create/update operations
- Thread safety via `Arc<Mutex<Vec<Product>>>`
- Poison-safe lock handling: recover inner state on mutex poison to avoid
  panic-induced denial-of-service while maintaining forward progress
- Comprehensive linting with Clippy (pedantic) and rustfmt
- Unit/integration tests: CRUD, filtering, concurrency, precision

## Configuration

The catalog enforces safe bounds with environment-driven configuration:

- `CATALOG_MAX_NAME_LEN` (default: 100, clamp: 1..=10_000)
- `CATALOG_MAX_STOCK` (default: 1_000_000, clamp: 0..=10_000_000)

Inputs are sanitized using these limits during creation and updates (name truncation; inventory clamped; negative prices coerced to zero).

## Thread Safety

- All catalog operations are guarded by a single `Mutex<Vec<Product>>` shared via `Arc`.
- ID generation is performed under a mutex to ensure sequential, unique IDs.
- Mutex poison is handled by recovering the inner state to avoid propagating panics.

## Governance

- Follows repository standards in `coding-guidelines.md` and `github-guidelines.md`
- Changes flow through feature branches only and PR review
- GitHub code scanning and local security tools validate changes on PRs

## Public API

- `ProductService::new()`
- `ProductService::create(NewProduct)` -> `Product`
- `ProductService::get_all()` -> `Vec<Product>`
- `ProductService::get_by_id(i32)` -> `Option<Product>`
- `ProductService::update_inventory(i32, i32)` -> `Option<Product>`
- `ProductService::filter(ProductFilter)` -> `Vec<Product>`
- `ProductService::delete(i32)` -> `bool`

See `tests/catalog.rs` and `tests/catalog_edge_cases.rs` for complete usage examples.

## Usage Example

```
use cto_parallel_test::catalog::{NewProduct, ProductFilter, ProductService};
use rust_decimal::Decimal;

let svc = ProductService::new();

// Create a product with precise decimal price
let apple = svc.create(NewProduct {
    name: "Apple".into(),
    description: "Fresh".into(),
    price: Decimal::new(199, 2), // 1.99
    inventory_count: 10,
});

// Update inventory
let _apple = svc.update_inventory(apple.id, 5).expect("updated");

// Filter by name and stock
let results = svc.filter(ProductFilter {
    name_contains: Some("app".into()),
    in_stock: Some(true),
    ..ProductFilter::default()
});
assert!(!results.is_empty());
```

## Filtering Tips

- `name_contains` is case-insensitive substring match
- `min_price`/`max_price` are inclusive bounds
- `in_stock = Some(true)` filters to `inventory_count > 0`; `Some(false)` filters to `inventory_count == 0`

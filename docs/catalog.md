# Product Catalog Module

This module provides a thread-safe, in-memory product catalog with:

- CRUD operations for `Product`
- Inventory updates with validation
- Flexible filtering by name, price, and stock
- Precise prices via `rust_decimal::Decimal`
- Auto-incrementing IDs using `AtomicI32`

## Security & Quality

- No hardcoded secrets or external IO
- Input validation for create/update operations
- Thread safety via `Arc<Mutex<Vec<Product>>>`
- Poison-safe lock handling: recover inner state on mutex poison to avoid
  panic-induced denial-of-service while maintaining forward progress
- Comprehensive linting with Clippy (pedantic) and rustfmt
- Unit/integration tests: CRUD, filtering, concurrency, precision

## Governance

- Follows repository standards in `coding-guidelines.md` and `github-guidelines.md`
- Changes flow through feature branches only and PR review
- Use GitHub code scanning for ongoing security checks on PRs

## Public API

- `ProductService::new()`
- `ProductService::create(NewProduct)` -> `Result<Product, CatalogError>`
- `ProductService::get_all()` -> `Vec<Product>`
- `ProductService::get_by_id(i32)` -> `Option<Product>`
- `ProductService::update_inventory(i32, i32)` -> `Result<Product, CatalogError>`
- `ProductService::filter(&ProductFilter)` -> `Vec<Product>`
- `ProductService::delete(i32)` -> `bool`

See `tests/catalog.rs` for complete usage examples.

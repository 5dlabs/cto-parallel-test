# Catalog Module

Provides a thread-safe in-memory product catalog with:
- CRUD operations
- Inventory management (non-negative stock)
  - Stock updates reject negatives; create clamps negatives to 0
- Flexible filtering by name, price range, and stock range
- Decimal precision prices via `rust_decimal`
- Auto-incrementing, thread-safe IDs

## Usage

```rust
use cto_parallel_test::catalog::{NewProduct, ProductFilter, ProductService};
use rust_decimal::Decimal;
use std::str::FromStr;

let svc = ProductService::new();
let p = svc.create(&NewProduct {
    name: "Widget".to_string(),
    price: Decimal::from_str("19.99").unwrap(),
    stock: 5,
});

let found = svc.get_by_id(p.id).unwrap();
assert_eq!(found.name, "Widget");

let filtered = svc.filter(&ProductFilter {
    min_price: Some(Decimal::from_str("10").unwrap()),
    max_price: Some(Decimal::from_str("20").unwrap()),
    ..ProductFilter::default()
});
assert_eq!(filtered.len(), 1);
```

## Thread Safety
- Internal storage uses `Arc<Mutex<Vec<Product>>>`
- ID generation uses `Arc<AtomicI32>`
- Lock poisoning is handled by continuing with the inner value

## Security & Quality
- No hardcoded secrets or external IO
- Input validation: stock updates reject negative values
- Input hygiene: create() trims name and clamps negative stock to 0
- Passes `cargo fmt`, `cargo clippy -W clippy::pedantic -D warnings`, and `cargo test`

### Code Scanning
- Repository includes a CodeQL workflow that runs on PRs to `main` and enforces zero MEDIUM/HIGH/CRITICAL alerts via `scripts/gh_code_scanning.sh`.
- Follow `github-guidelines.md` for the mandatory PR creation step so code scanning can execute.
- Reference `coding-guidelines.md` for linting and testing gates to pass prior to PR.

Refer to `coding-guidelines.md` and `github-guidelines.md` for project-wide standards.

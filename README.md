# cto-parallel-test

Thread-safe, in-memory product catalog for an e-commerce API. The catalog exposes CRUD operations, inventory updates, and flexible filtering while preserving decimal precision for prices.

- Thread-safe storage via `Arc<Mutex<Vec<Product>>>`
- Auto-incrementing product IDs
- Decimal prices using `rust_decimal::Decimal`
- Filtering by name (case-insensitive), price bounds, and stock state
- Input sanitization with environment-driven bounds
- `#![forbid(unsafe_code)]` at crate level

## Module

- `src/catalog/models.rs`: `Product`, `NewProduct`, `ProductFilter` and safe configuration helpers
- `src/catalog/service.rs`: `ProductService` implementing CRUD, inventory, and filtering
- `src/catalog/mod.rs`: module wiring and public re-exports
- `src/lib.rs`: crate root with module export

## Configuration

Environment overrides with safe clamps are supported:
- `CATALOG_MAX_NAME_LEN` (default 100, clamp 1..=10_000)
- `CATALOG_MAX_DESCRIPTION_LEN` (default 1_000, clamp 1..=50_000)
- `CATALOG_MAX_STOCK` (default 1_000_000, clamp 0..=10_000_000)

## Quickstart

```rust
use cto_parallel_test::catalog::{NewProduct, ProductFilter, ProductService};
use rust_decimal::Decimal;

fn main() {
    // Initialize service (thread-safe in-memory store)
    let svc = ProductService::new();

    // Create a product with exact decimal precision price
    let apple = svc.create(&NewProduct {
        name: "Apple".to_string(),
        description: "Crisp and fresh".to_string(),
        price: Decimal::new(199, 2), // 1.99
        inventory_count: 10,
    });

    // Update inventory
    let _updated = svc.update_inventory(apple.id, 5);

    // Filter by case-insensitive name, price range and in-stock
    let results = svc.filter(ProductFilter {
        name_contains: Some("app".into()),
        min_price: Some(Decimal::new(100, 2)),
        max_price: Some(Decimal::new(300, 2)),
        in_stock: Some(true),
    });

    assert_eq!(1, results.len());
}
```

## Development

- Format: `cargo fmt --all -- --check`
- Lint: `cargo clippy --workspace --all-targets --all-features -- -D warnings -W clippy::pedantic`
- Test: `cargo test --workspace --all-features`

## Security

- Secrets: `gitleaks detect -c .gitleaks.toml -f json -r gitleaks_report.json`
- Dependencies: `cargo install cargo-audit --locked && cargo audit`
- CodeQL and SARIF uploads run in CI for PRs to `main`

## Pull Requests

Create a PR from `feature/task-4-implementation` to `main`:

```
# Ensure your branch is pushed
git push -u origin feature/task-4-implementation

# Create PR with required labels (requires GH_TOKEN)
gh pr create \
  --base main \
  --head feature/task-4-implementation \
  --title "feat: product catalog module with thread-safe storage, filtering, and tests" \
  --body "Implements thread-safe product catalog with CRUD, inventory, filtering, and tests. Security scans and quality gates passing." \
  --label task-4 --label service-cto-parallel-test --label run-play-task-4-zfnqr
```

After the PR is open, check GitHub Code Scanning alerts for the PR:

```
PR=$(gh pr view --json number -q .number)
gh api \
  "/repos/5dlabs/cto-parallel-test/code-scanning/alerts?state=open&pr=${PR}" \
  | jq '[.[] | {rule: (.rule.id // .rule.name), severity: ((.rule.severity // .rule.security_severity_level // "unknown")|ascii_downcase), state: .state, url: .html_url}]'
```

Refer to `coding-guidelines.md` and `github-guidelines.md` for repository policies and pre-PR quality gates.

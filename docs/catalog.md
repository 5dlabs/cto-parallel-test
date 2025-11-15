% Product Catalog Module

Thread-safe, in-memory product catalog with CRUD, inventory management, and flexible filtering.

- Storage: `Arc<Mutex<Vec<Product>>>` (no `unsafe`)
- IDs: Auto-incrementing `i32` starting at 1
- Prices: `rust_decimal::Decimal` for exact precision
- Filtering: name (case-insensitive substring), price range, stock state
- Concurrency: safe mutations via internal locking; snapshots returned by value

## Models
- `Product { id, name, description, price, inventory_count }`
- `NewProduct { name, description, price, inventory_count }`
- `ProductFilter { name_contains, min_price, max_price, in_stock }`

## Service
- `ProductService::new()`
- `create(&NewProduct) -> Product`
- `get_all() -> Vec<Product>`
- `get_by_id(i32) -> Option<Product>`
- `update_inventory(i32, i32) -> Option<Product>`
- `filter(ProductFilter) -> Vec<Product>`
- `delete(i32) -> bool`

## Configuration (Environment Variables)
Inputs are validated and bounded to safe caps; all limits are externally configurable.

- `CATALOG_MAX_NAME_LEN` (default 100, clamp 1..=10_000)
- `CATALOG_MAX_DESCRIPTION_LEN` (default 1_000, clamp 1..=50_000)
- `CATALOG_MAX_STOCK` (default 1_000_000, clamp 0..=10_000_000)

Example:
```
export CATALOG_MAX_NAME_LEN=120
export CATALOG_MAX_DESCRIPTION_LEN=5000
export CATALOG_MAX_STOCK=250000
```

## Security Notes
- No hardcoded secrets; configuration via environment only
- Input clamping on strings and integers to avoid resource exhaustion
- No filesystem or command execution; no deserialization of untrusted types
- `#![forbid(unsafe_code)]` enforced at crate level

## Running Quality Gates
```
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings -W clippy::pedantic
cargo test --workspace --all-features
```

## Local Security Scans
```
# Secrets
gitleaks detect -c .gitleaks.toml -f json -r gitleaks_report.json

# Dependency advisories
cargo install cargo-audit --locked || true
cargo audit
```

## GitHub Code Scanning
After opening a PR from `feature/task-4-implementation` to `main`:
```
PR=$(gh pr view --json number -q .number)
gh api \
  "/repos/5dlabs/cto-parallel-test/code-scanning/alerts?state=open&pr=${PR}" \
  | jq '[.[] | {rule: (.rule.id // .rule.name), severity: ((.rule.severity // .rule.security_severity_level // "unknown")|ascii_downcase), state: .state, url: .html_url}]'
```

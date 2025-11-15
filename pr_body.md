## Implementation Summary
Implemented a thread-safe in-memory product catalog with CRUD operations, inventory management, and flexible filtering. Prices use rust_decimal for exact precision. Added comprehensive tests for CRUD, filtering, concurrency, and precision. Security scans (gitleaks, cargo-audit) show zero findings. CI includes CodeQL and dependency/secret scanners.

## Changes Made
- Added `catalog` module with `models` and `service`
- Implemented `Product`, `NewProduct`, `ProductFilter` using `rust_decimal::Decimal`
- Implemented `ProductService` with `Arc<Mutex<...>>`, auto-increment IDs, CRUD, inventory updates, and filtering
- Parameterized bounds via `CATALOG_MAX_NAME_LEN`, `CATALOG_MAX_DESCRIPTION_LEN`, `CATALOG_MAX_STOCK` with safe clamps
- Added tests for CRUD, filtering, concurrency, and precision
- Updated README and SECURITY docs; ensured CI has CodeQL and scanners

## Testing Performed
- cargo fmt (check): PASS
- cargo clippy (pedantic, deny warnings): PASS
- cargo test (workspace, all-features): PASS
- gitleaks: no leaks found
- cargo audit: no vulnerabilities found

## Notes
- No unsafe code
- No external dependencies beyond serde and rust_decimal
- CI will surface any CodeQL alerts on the PR

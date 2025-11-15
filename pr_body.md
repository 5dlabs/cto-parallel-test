## Implementation Summary
Implemented a thread-safe in-memory product catalog with CRUD operations, inventory management, and flexible filtering. Prices use rust_decimal for exact precision. Added comprehensive tests for CRUD, filtering, concurrency, and precision. Security scans (gitleaks, cargo-audit) show zero findings. CI includes CodeQL and dependency/secret scanners.

## Changes Made
- `catalog` module with `models` and `service`
- `Product`, `NewProduct`, `ProductFilter` using `rust_decimal::Decimal`
- `ProductService` with `Arc<Mutex<...>>`, auto-increment IDs, CRUD, inventory updates, and filtering
- Bounds via env vars with safe clamps: `CATALOG_MAX_NAME_LEN`, `CATALOG_MAX_DESCRIPTION_LEN`, `CATALOG_MAX_STOCK`
- Tests for CRUD, filtering, concurrency, and precision
- Docs: README and SECURITY updated; CI includes CodeQL + scanners

## Testing Performed
- cargo fmt — PASS
- cargo clippy (pedantic, -D warnings) — PASS
- cargo test (workspace, all-features) — PASS
- gitleaks — PASS (0 findings)
- cargo audit — PASS (0 vulns)

## Notes
- No unsafe code
- In-memory only; no external services
- CI will surface any CodeQL alerts on the PR

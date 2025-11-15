Implements a thread-safe, in-memory product catalog module with:

- CRUD operations (create, list, get, delete)
- Inventory updates (safe clamped upper-bounds, negative backorders allowed)
- Flexible filtering: case-insensitive name substring, min/max price, in-stock flag
- Decimal precision for prices via `rust_decimal::Decimal`
- Auto-incrementing product IDs (thread-safe)
- Input sanitization and environment-driven bounds:
  - `CATALOG_MAX_NAME_LEN` (1..=10_000; default 100)
  - `CATALOG_MAX_DESCRIPTION_LEN` (1..=50_000; default 1_000)
  - `CATALOG_MAX_STOCK` (0..=10_000_000; default 1_000_000)

Quality gates:
- `cargo fmt` (check): passing
- `cargo clippy` (pedantic, deny warnings): passing
- `cargo test` (workspace): all tests passing

Security:
- Repo includes CodeQL and cargo-audit in CI
- Local secret scan via gitleaks: no leaks detected
- No unsafe Rust; `#![forbid(unsafe_code)]`
- No hardcoded secrets; secure defaults and input bounds

This PR adheres to coding and GitHub guidelines in the repository.

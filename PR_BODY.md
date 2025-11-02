## Implementation Summary
Implemented a thread-safe, in-memory product catalog module with CRUD, inventory management, and flexible filtering. Prices use `rust_decimal` for precision. Added comprehensive unit tests including concurrency checks and documentation.

## Changes Made
- Added `src/catalog` module with models and `ProductService`
- Implemented auto-incrementing IDs and thread-safe storage
- Added filtering by name, price range, and stock range
- Enforced non-negative stock updates (secure default)
- Added `rust_decimal`, `serde`, and `serde_json` dependencies
- Configured `clippy.toml` per project guidelines
- Wrote unit tests for CRUD, filtering, concurrency, and decimal precision
- Added docs at `docs/catalog.md`

## Security Review
- No I/O, no shell/process spawning, no file/network access
- No unsafe Rust; mutex poison handled without panicking
- No hardcoded secrets or credentials
- Input validation: inventory updates reject negative values
- Serialization derives used only for in-memory data models
- Concurrency: internal storage guarded by `Arc<Mutex<...>>`; IDs via `Arc<AtomicI32>`

## Testing Performed
- Ran `cargo fmt --all -- --check`
- Ran `cargo clippy --workspace --all-targets --all-features -- -D warnings -W clippy::pedantic`
- Ran `cargo test --workspace --all-features` (all tests pass)

## Notes
- Pure in-memory catalog; safe locking semantics and secure defaults applied.

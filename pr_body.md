Implements a thread-safe, in-memory product catalog for the e-commerce API with:

- Thread-safe storage via `Arc<Mutex<Vec<Product>>>`
- CRUD operations and inventory updates
- Flexible filtering (name substring, price bounds, stock state)
- Decimal price precision (`rust_decimal`)
- Auto-incrementing product IDs
- Input bounds enforced via environment variables with safe caps
- `#![forbid(unsafe_code)]`

Security & quality:
- `cargo fmt`, `clippy -D warnings -W clippy::pedantic`, and tests all pass
- `gitleaks` scan: clean (no findings)
- `cargo audit`: no vulnerable dependencies

Usage docs and commands in `README.md`.

Labels: task-4, service-cto-parallel-test, run-play-task-4-zfnqr

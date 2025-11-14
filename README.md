# cto-parallel-test

Thread-safe in-memory product catalog for e-commerce use cases.

- CRUD operations for products
- Inventory updates with validation
- Flexible filtering (name, price, stock)
- Precise prices with `rust_decimal::Decimal`
- Auto-incrementing product IDs
- Concurrency-safe via `Arc<Mutex<Vec<Product>>>` with poison recovery

See `docs/catalog.md` for full API details and examples.

## Security & Quality

- Follows repository standards in `coding-guidelines.md` and `github-guidelines.md`
- No unsafe Rust; input validation on create/update
- Linting: `cargo fmt` and `cargo clippy -W clippy::pedantic -D warnings`
- Tests: CRUD, filtering, concurrency, decimal precision
- CI: CodeQL, gitleaks, OSV, and `cargo-audit` enabled under `.github/workflows/`

## Local Development

- Format: `cargo fmt --all`
- Lint: `cargo clippy --workspace --all-targets --all-features -- -D warnings -W clippy::pedantic`
- Test: `cargo test --workspace --all-features`

## Security Scanning

Local checks:

- Secrets: `gitleaks detect -c .gitleaks.toml --no-git -f json -r gitleaks_report_latest.json`
- Dependencies: `cargo audit -D warnings`

GitHub code scanning is configured via Actions and will run on PRs and pushes to `feature/task-4-implementation` and `main`.


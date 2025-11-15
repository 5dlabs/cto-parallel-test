Implements a thread-safe in-memory product catalog module with:
- CRUD operations and auto-incrementing IDs
- Inventory tracking and updates
- Flexible filtering by name (case-insensitive), price bounds, and stock state
- Decimal precision for prices via rust_decimal::Decimal
- Safe input bounds configurable via environment variables

Security & Quality:
- No unsafe code; input sanitization and safe clamps
- cargo fmt/clippy/tests passing
- cargo-audit: 0 vulnerabilities
- gitleaks: no findings

Docs updated in README with usage, configuration, and CI notes.

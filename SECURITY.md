# Security Posture

This repository enforces secure defaults and continuous security scanning across local and CI workflows.

- No unsafe Rust: the crate root uses `#![forbid(unsafe_code)]`.
- Input validation: lengths and counts are bounded using environment-configured limits with absolute caps to prevent memory abuse.
- Decimal precision: monetary values use `rust_decimal::Decimal` to avoid floating-point rounding issues.
- Secrets hygiene: `gitleaks` runs locally and in CI; findings fail the job.
- Dependency hygiene: `cargo-audit` and OSV scans run locally and in CI; findings fail the job.
- Code Scanning: GitHub CodeQL and SARIF uploads are enabled in `.github/workflows`.

## Local Security Checks

Run before opening a PR:

```
# Formatting, linting, tests
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings -W clippy::pedantic
cargo test --workspace --all-features

# Secrets and dependencies
gitleaks detect --no-banner -s . -f json -r gitleaks_report.json
cargo install cargo-audit --locked && cargo audit --json > cargo_audit_report.json
```

## GitHub Code Scanning on PRs

After pushing the branch and creating a PR, fetch open alerts for that PR:

```
# Requires GH_TOKEN to be set (GitHub App token)
PR=$(gh pr view --json number -q .number)
OWNER_REPO=$(gh repo view --json nameWithOwner -q .nameWithOwner)

gh api \
  "/repos/${OWNER_REPO}/code-scanning/alerts?state=open&pr=${PR}"
```

## Configuration (Environment)

- `CATALOG_MAX_NAME_LEN` (default 100, clamp 1..=10_000)
- `CATALOG_MAX_DESCRIPTION_LEN` (default 1_000, clamp 1..=50_000)
- `CATALOG_MAX_STOCK` (default 1_000_000, clamp 0..=10_000_000)

These values are validated and clamped to safe ranges to prevent denial-of-service via unbounded allocations.

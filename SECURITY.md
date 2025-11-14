# Security Policy and Scanning

This repository enforces security-by-default. Changes are scanned locally and in CI before merge.

## Local Scans (run before PR)

- Secrets: `gitleaks detect --no-git -c .gitleaks.toml -f json -r gitleaks_report_latest.json`
- Dependencies: `osv-scanner --lockfile=Cargo.lock --format json --output osv_report.json`
- Optional: `cargo audit -D warnings` (if installed)

## CI Scans (automatic on PR and push)

- CodeQL: `.github/workflows/codeql.yml`
- Secrets (gitleaks) + SARIF upload: `.github/workflows/security-scans.yml`
- OSV dependency scan + SARIF upload: `.github/workflows/security-scans.yml`
- RustSec audit: `.github/workflows/ci.yml`

## Module Hardening Summary

- No unsafe Rust; locks are poison-safe via `PoisonError::into_inner`
- Input validation for product creation and stock updates
- Defensive bounds on untrusted input:
  - `name` must be non-empty and at most 100 chars
  - `stock` must be between 0 and 1,000,000 (inclusive)
- Decimal arithmetic uses `rust_decimal` to avoid floating-point rounding errors
- No external IO, network, or paths; no deserialization of untrusted types beyond `serde` structs

## GitHub Code Scanning Alerts

To list open alerts for a PR (requires auth via `GH_TOKEN`):

```
export GH_REPO="5dlabs/cto-parallel-test"
export PR_NUMBER="<PR_NUMBER>"
gh api "/repos/${GH_REPO}/code-scanning/alerts?state=open&pr=${PR_NUMBER}"
```

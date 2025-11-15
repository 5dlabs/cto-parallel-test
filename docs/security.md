Security Scanning and Quality Gates

Overview
- Local scans: gitleaks and cargo-audit
- GitHub Code Scanning (CodeQL) runs in CI via .github/workflows/codeql.yml
- CI also runs clippy, fmt, tests, gitleaks, and cargo-audit

Local Security Scan
```bash
# Optional: install tools if missing
cargo install cargo-audit --locked || true

# Run local scans
scripts/security-scan.sh
```

GitHub Code Scanning (PR Alerts)
- The security script attempts to query open alerts for the current PR using GH_TOKEN.
- Requirements:
  - Environment variable GH_TOKEN must be set to a GitHub App or PAT with code scanning scope
  - A pull request must exist for the current branch

Manual API call (fallback)
```bash
OWNER_REPO=$(git config --get remote.origin.url | sed -E 's#(git@|https://)github.com[:/ ]##; s/\\.git$//')
PR_NUMBER=$(gh pr list --head "$(git rev-parse --abbrev-ref HEAD)" --json number -q '.[0].number')
gh api -H "Authorization: Bearer $GH_TOKEN" \
  "/repos/$OWNER_REPO/code-scanning/alerts?state=open&pr=$PR_NUMBER" | jq .
```

Secure Defaults & Practices
- No hardcoded credentials; use environment variables (.env is gitignored)
- Diesel queries are parameterized by design
- Price uses NUMERIC in SQL and BigDecimal in Rust for precision
- Database constraints enforce domain invariants:
  - Non-negative price and inventory
  - Positive cart item quantity
  - Unique (cart_id, product_id)

Quality Gates (must pass before PR)
```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings -W clippy::pedantic
cargo test --workspace --all-features
cargo audit
gitleaks detect --no-banner --no-git --source .
```


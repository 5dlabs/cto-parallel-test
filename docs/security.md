Security Scanning Overview

- CI runs three security jobs on pushes and PRs to `main`:
  - Secret scan: gitleaks (uploads SARIF to GitHub Code Scanning)
  - Dependency scan: OSV-Scanner (uploads SARIF; fails on findings)
  - RustSec audit: cargo-audit (fails on advisories)

Checking alerts on a PR (requires gh auth):

- Preferred non-interactive auth: export `GH_TOKEN` with a GitHub App/Action token.
  - Example: `export GH_TOKEN=ghp_xxx` (or rely on CI-provided token)
- Alternatively, interactive auth: `gh auth login --hostname github.com`
- Create a PR with required labels (feature branch only):
  - `gh pr create --fill --head feature/task-4-implementation --base main --label task-4 --label service-cto-parallel-test --label run-play-task-4-l4m2j`
- Get PR number: `PR=$(gh pr view --json number -q .number)`
- List open alerts: `gh api "/repos/5dlabs/cto-parallel-test/code-scanning/alerts?state=open&pr=${PR}" | jq '[.[] | {rule: .rule.id, severity: (.rule.severity // .rule.security_severity_level // "unknown"), state: .state, url: .html_url}]'`
- Enforce policy (MEDIUM/HIGH/CRITICAL):
  `gh api "/repos/5dlabs/cto-parallel-test/code-scanning/alerts?state=open&pr=${PR}" | jq -e 'map(select((.rule.severity // .rule.security_severity_level) | IN("medium","high","critical"))) | if length==0 then empty else . end' || echo "No MEDIUM/HIGH/CRITICAL alerts"`

Local pre-PR checks:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings -W clippy::pedantic`
- `cargo test --workspace --all-features`

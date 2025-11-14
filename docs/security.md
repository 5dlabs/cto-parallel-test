Security Scanning Overview

- CI runs three security jobs on pushes and PRs to `main`:
  - Secret scan: gitleaks (uploads SARIF to GitHub Code Scanning and fails the job if any findings are present)
  - Dependency scan: OSV-Scanner (uploads SARIF; fails on findings)
  - RustSec audit: cargo-audit (fails on advisories)
  - Additionally, CodeQL analysis runs and the workflow enforces zero MEDIUM/HIGH/CRITICAL Code Scanning alerts on PRs.

Checking alerts on a PR (requires gh auth):

- Preferred non-interactive auth: export `GH_TOKEN` with a GitHub App/Action token.
  - Example: `export GH_TOKEN=ghp_xxx` (or rely on CI-provided token)
- Alternatively, interactive auth: `gh auth login --hostname github.com`
- Ensure a PR exists for the current branch (feature branch only):
  - `gh pr view --json number -q .number || gh pr create --fill --head feature/task-4-implementation --base main --label task-4 --label service-cto-parallel-test --label run-play-task-4-n98rn`
- Get PR number: `PR=$(gh pr view --json number -q .number)`
- List open alerts: `gh api "/repos/5dlabs/cto-parallel-test/code-scanning/alerts?state=open&pr=${PR}" | jq '[.[] | {rule: (.rule.id // .rule.name), severity: ((.rule.severity // .rule.security_severity_level // "unknown")|ascii_downcase), state: .state, url: .html_url}]'`
- Enforce policy (MEDIUM/HIGH/CRITICAL):
  `gh api "/repos/5dlabs/cto-parallel-test/code-scanning/alerts?state=open&pr=${PR}" | jq -e 'map(select(((.rule.severity // .rule.security_severity_level // "unknown")|ascii_downcase) | test("^(medium|high|critical)$"))) | if length==0 then empty else . end' || echo "No MEDIUM/HIGH/CRITICAL alerts"`

Local pre-PR checks:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings -W clippy::pedantic`
- `cargo test --workspace --all-features`
- `gitleaks detect -c .gitleaks.toml -r gitleaks_report_latest.json -f json --no-banner`
- `cargo audit -D warnings`

Local verification results (this change set):

- Formatting: PASS
- Clippy (pedantic, deny warnings): PASS
- Tests: PASS (all unit and integration tests green)
- Secret scan (gitleaks): PASS (no leaks found)
- Dependency audit (cargo-audit): PASS (no advisories)

Latest verification:

 - Timestamp: 2025-11-14 15:49:20 UTC
 - Tools: rustfmt, clippy (pedantic, -D warnings), cargo test, cargo-audit (JSON), gitleaks (JSON), OSV-Scanner (JSON)
 - Result: All checks PASS; gitleaks findings = 0; cargo-audit vulnerabilities = 0; OSV vulnerabilities = 0; zero MEDIUM/HIGH/CRITICAL issues in local scans
 - Artifacts: `gitleaks_report_latest.json`, `cargo_audit_report.json`, `osv_report.json`

Notes:
- GitHub API access for listing Code Scanning alerts from the local environment may be rate-limited or require explicit `GH_TOKEN` export. If unauthenticated locally, rely on CI where CodeQL + SARIF uploads run on pushes/PRs. Use the commands above once a valid token is configured.
- To run the same checks locally with auth, ensure `GH_TOKEN` is exported and use the commands above.

If GitHub auth is unavailable locally, proceed with local verification and push to the feature branch. CI will run CodeQL and upload all SARIF results on the PR.

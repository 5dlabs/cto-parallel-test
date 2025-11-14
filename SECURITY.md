# Security Overview

This repository includes a frontend e‑commerce app with secure defaults. Key practices applied:

- Input validation and encoding
  - API paths normalized and path segments encoded in `frontend/src/config.js`
  - Route params validated in `frontend/src/pages/ProductDetail.jsx`
- XSS prevention
  - No usage of `dangerouslySetInnerHTML`
  - Strict Content Security Policy added in `frontend/index.html`
- Sensitive data handling
  - No secrets committed; configuration via environment variables (`.env`)
  - `localStorage` only stores non‑sensitive cart state
- Dependency hygiene
  - `npm audit` workflow included (`npm run audit:ci`) to check prod dependencies; threshold configurable via `AUDIT_LEVEL` env
- Secure defaults
  - CSP, Referrer‑Policy, X‑Content‑Type‑Options, X‑Frame‑Options set via meta tags

## Code Scanning

Code scanning for the PR can be queried with GitHub CLI once authenticated:

PR_NUM=$(gh pr list --head feature/task-6-implementation --json number -q '.[0].number')
gh api \
  "/repos/5dlabs/cto-parallel-test/code-scanning/alerts?state=open&pr=$PR_NUM" \
  --jq '.[] | {rule: .rule.id, severity: .rule.severity, path: .most_recent_instance.location.path, start: .most_recent_instance.location.start_line}'

Fix all MEDIUM/HIGH/CRITICAL findings before merging.

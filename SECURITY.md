# Security Overview

This repository includes a frontend e‑commerce app with secure defaults. Key practices applied:

- Input validation and encoding
  - API base URL normalized and path segments encoded in `frontend/src/config.js`
  - IDs and route params validated/sanitized where applicable
- XSS prevention
  - No usage of `dangerouslySetInnerHTML`
  - Strict Content Security Policy added in `frontend/index.html`
  - Next.js headers enforce CSP without `unsafe-inline` for styles in `frontend/next.config.ts`
  - `connect-src` restricted to the configured API origin when `NEXT_PUBLIC_API_BASE_URL`/`VITE_API_BASE_URL` is set (see `frontend/next.config.ts`)
- Sensitive data handling
  - No secrets committed; configuration via environment variables (`.env`)
  - `localStorage` only stores non‑sensitive cart state
- Dependency hygiene
  - `npm audit` executed in CI (see `.github/workflows/frontend-ci.yml`) to check prod dependencies; threshold set to fail on moderate+
- Secure defaults
  - CSP, Referrer‑Policy, X‑Content‑Type‑Options, X‑Frame‑Options set via headers/meta

## Code Scanning

Code scanning for the PR can be queried with GitHub CLI once authenticated:

PR_NUM=$(gh pr list --head feature/task-6-implementation --json number -q '.[0].number')
gh api \
  "/repos/5dlabs/cto-parallel-test/code-scanning/alerts?state=open&pr=$PR_NUM" \
  --jq '.[] | {rule: .rule.id, severity: .rule.severity, path: .most_recent_instance.location.path, start: .most_recent_instance.location.start_line}'

Fix all MEDIUM/HIGH/CRITICAL findings before merging.

If GitHub CLI authentication is unavailable in your environment (e.g., `HTTP 401`), authenticate and re‑run the query (GitHub App token supported via `GH_TOKEN`):

```
export GH_TOKEN=<GITHUB_APP_TOKEN>
gh auth status || gh auth login -h github.com --with-token <<<"$GH_TOKEN"
PR_NUM=$(gh pr list --head feature/task-6-implementation --json number -q '.[0].number')
gh api "/repos/5dlabs/cto-parallel-test/code-scanning/alerts?state=open&pr=$PR_NUM" \
  --jq '.[] | {rule: .rule.id, severity: .rule.severity, path: .most_recent_instance.location.path, start: .most_recent_instance.location.start_line}'
```

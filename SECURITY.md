# Security Overview

This repository includes a frontend e‑commerce app with secure defaults. Key practices applied:

- Input validation and encoding
  - API base normalization and safe URL building in `frontend/lib/config.ts`
  - Route params validated and user input trimmed in pages under `frontend/app/**` and `frontend/src/pages/**`
- XSS prevention
  - No usage of `dangerouslySetInnerHTML` or unsafe DOM sinks
  - Strict Content Security Policy via meta tag in `frontend/index.html` and via headers in `frontend/Dockerfile` (nginx)
- Sensitive data handling
  - No secrets committed; configuration via environment variables (`.env`, `.env.example` provided)
  - `localStorage` only stores non‑sensitive cart state
- Dependency hygiene
  - `npm audit` CI gate checks runtime dependencies (moderate+ fails)
  - Local audits tracked in `security/npm-audit.json` and `security/npm-audit-full.json`
- Secure defaults
  - CSP, Referrer‑Policy, X‑Content‑Type‑Options, X‑Frame‑Options enforced
  - Docker nginx adds HSTS, COOP/CORP, and denies object/embed; runs as non‑root user
  - Docker build hardening: `.dockerignore` excludes `.env*`, `node_modules`, VCS/CI files to prevent secrets from entering images

## Code Scanning

Automated scanners in CI/CD:
- CodeQL: `.github/workflows/codeql.yml`
- Secrets scan (gitleaks): `.github/workflows/secrets-scan.yml`
- Frontend CI (lint/build/audit gate): `.github/workflows/frontend-ci.yml`

Query open code scanning alerts for this PR with GitHub CLI once authenticated:

```
PR_NUM=$(gh pr list --head feature/task-6-implementation --json number -q '.[0].number')
gh api \
  "/repos/5dlabs/cto-parallel-test/code-scanning/alerts?state=open&pr=$PR_NUM" \
  --jq '.[] | {rule: .rule.id, severity: .rule.severity, path: .most_recent_instance.location.path, start: .most_recent_instance.location.start_line}'
```

If GitHub CLI authentication is unavailable, authenticate and re-run the query:

```
gh auth login -h github.com
PR_NUM=$(gh pr list --head feature/task-6-implementation --json number -q '.[0].number')
gh api \
  "/repos/5dlabs/cto-parallel-test/code-scanning/alerts?state=open&pr=$PR_NUM" \
  --jq '.[] | {rule: .rule.id, severity: .rule.severity, path: .most_recent_instance.location.path, start: .most_recent_instance.location.start_line}'
```

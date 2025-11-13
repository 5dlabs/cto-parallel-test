Cipher Security Scanner – Service: cto-parallel-test

Overview
- Security focus: prevent XSS, path traversal, command/SQL injection, and hardcoded secrets.
- Frontend uses parameterized API URL, encodes path segments, validates route ids, and stores only cart items in localStorage.

Local Checks
- Lint: `cd frontend && npm ci && npm run lint`
- Build: `npm run build`
- Audit: `npm run audit:ci` (fails on high/critical)

GitHub Code Scanning
- CodeQL workflow in `.github/workflows/codeql.yml` runs on pushes and PRs.
- Frontend CI in `.github/workflows/frontend-ci.yml` runs lint/build/audit per PR.
- Query open PR alerts once PR is created:
  gh api "/repos/5dlabs/cto-parallel-test/code-scanning/alerts?state=open&pr=<PR_NUMBER>" \
    --jq '.[] | {rule: .rule.id, severity: .rule.severity, path: .most_recent_instance.location.path, start: .most_recent_instance.location.start_line}'

Secrets
- Do not commit secrets. Configure `VITE_API_BASE_URL` via environment. See `frontend/.env.example`.


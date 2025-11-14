Verification Snapshot – Attempt 36 (cto-parallel-test)

Date: 2025-11-14 (UTC)

Local Results
- Frontend install/build: npm ci, npm run lint, npm run build — PASSED
- Runtime dependency audit: npm audit --omit=dev --audit-level=moderate — 0 vulnerabilities
- Full dependency audit: npm audit — 0 vulnerabilities
- Secrets scan: gitleaks detect — no leaks found

Notes
- Dev server: npm start (Vite) on port 3000; port configurable via VITE_PORT/PORT
- API base URL parameterized via VITE_API_BASE_URL; path segments encoded and ids validated
- CSP hardened in index.html; dangerous sinks disallowed via ESLint security rules

GitHub Code Scanning
- Will run in CI (CodeQL workflow). CLI query requires auth; run after PR is created:
  gh api "/repos/5dlabs/cto-parallel-test/code-scanning/alerts?state=open&pr=<PR_NUMBER>" \
    --jq '.[] | {rule: .rule.id, severity: .rule.severity, path: .most_recent_instance.location.path, start: .most_recent_instance.location.start_line}'


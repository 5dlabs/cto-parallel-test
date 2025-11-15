Task 6: Frontend Implementation (cto-parallel-test)

Summary
- Implemented a complete React frontend using Vite, Tailwind CSS, and shadcn/ui.
- Pages include: Home, Product List, Product Detail, Cart, Login, Register.
- Routing via `react-router-dom`; Header includes cart badge; Footer present on all pages.
- Environment-driven configuration: API base URL parameterized via `VITE_API_BASE_URL`.
- Dev server port parameterized via `VITE_PORT`/`PORT`; audit threshold via `AUDIT_LEVEL`.

Security
- No hardcoded secrets; `.env.example` provided.
- API paths constructed with `encodeURIComponent` to prevent path traversal.
- IDs validated/normalized in React and Next product detail routes.
- Local storage limited to cart items only; no sensitive data persisted.
- CodeQL workflow configured; Frontend CI performs lint/build/audit.
- Removed sensitive console logging in Next login/register pages; forms now call real API endpoints configured via env.
- Replaced Next mock product/cart data with live API fetch and client cart state (no mocks left in repo).
- CSP hardened in `frontend/index.html` (no `style-src 'unsafe-inline'`, added `object-src 'none'`).

How to Run
- `cd frontend`
- `cp .env.example .env` and set `VITE_API_BASE_URL`
- `npm ci`
- `npm start` and open `http://localhost:3000`

Code Scanning
- After opening a PR, query open alerts:
- `gh api "/repos/5dlabs/cto-parallel-test/code-scanning/alerts?state=open&pr=<PR_NUMBER>" --jq '.[] | {rule: .rule.id, severity: .rule.severity, path: .most_recent_instance.location.path, start: .most_recent_instance.location.start_line}'`

Notes
- Dev server bound to port 3000 via `npm start`.
- Aliases configured: `@` -> `src` (see `vite.config.js`, `jsconfig.json`).
 
Local Validation (this run)
- `npm ci`, `npm run lint`, and `npm run build` completed successfully.
- `npm audit --omit=dev` reported 0 vulnerabilities (see `security/npm-audit.json`).
- Full audit reported 0 vulnerabilities (see `security/npm-audit-full.json`).
- `gitleaks` reported no secrets; report: `security/gitleaks-report.json`.

# Cipher Shop Frontend (React + Vite + shadcn/ui)

Production-ready e-commerce frontend using React, Vite, Tailwind CSS, and shadcn/ui. Implements secure defaults with parameterized API endpoints and client-side input validation.

## Quick Start

- cp .env.example .env and set `VITE_API_BASE_URL` (e.g., `https://api.example.com`)
  - Optional runtime knobs:
    - `VITE_FREE_SHIPPING_THRESHOLD` (number, default 50)
    - `VITE_STANDARD_SHIPPING_COST` (number, default 9.99)
  - Next.js app also supports `NEXT_PUBLIC_API_BASE_URL`, `NEXT_PUBLIC_FREE_SHIPPING_THRESHOLD`, and `NEXT_PUBLIC_STANDARD_SHIPPING_COST`.
- Optionally set `VITE_PORT`/`PORT` to change dev port (default: 3000)
- npm install
- npm start
- Open http://localhost:3000

## Features

- Routing: Home, Product List, Product Detail, Cart, Login, Register
- UI: Header with cart badge, Footer; shadcn/ui components (button, card, badge, input, form, navigation-menu)
- Styling: Tailwind CSS with shadcn/ui tokens (New York theme)
- Config: All endpoints come from `VITE_API_BASE_URL` (no hardcoded URLs)
  - Thresholds for shipping are parameterized via env

## Security Notes

- Content Security Policy (CSP) set in `index.html` to restrict sources:
  - default-src 'self'
  - script-src 'self'
  - style-src 'self'
  - img-src 'self' data: blob: https:
  - font-src 'self' https: data:
  - connect-src 'self' https:
  - object-src 'none'
  - frame-ancestors 'none'
  - base-uri 'self'
  - form-action 'self'
- Additional meta headers: `Referrer-Policy: no-referrer`, `X-Content-Type-Options: nosniff`, `X-Frame-Options: DENY`
- Path handling: API URLs constructed with encoded path segments (`src/config.js`) to prevent traversal/injection.
- API base URL validation: only `http`/`https` schemes are allowed; invalid or unsafe schemes are ignored.
- Route params validated (e.g., `ProductDetail.jsx` allows only safe IDs).
- No `dangerouslySetInnerHTML`; forms trim inputs; no credentials stored in localStorage.
- Dependency audit: `npm run audit:ci` fails on issues at `AUDIT_LEVEL` (default: moderate).

## Scripts

- `npm start` – Dev server on configurable port (`VITE_PORT`/`PORT`, default 3000)
- `npm run build` – Production build to `dist/`
- `npm run lint` – ESLint checks
- `npm run audit:ci` – Dependency audit (runtime deps only); set `AUDIT_LEVEL` to choose severity threshold

## Configuration

- Alias `@` → `src` configured in `vite.config.js` and `jsconfig.json`
- Tailwind config in `tailwind.config.js`; base styles in `src/index.css`

## Code Scanning

CodeQL and CI are configured in `.github/workflows`. Once the PR is open, check alerts:

```
gh api "/repos/5dlabs/cto-parallel-test/code-scanning/alerts?state=open&pr=<PR_NUMBER>"
```

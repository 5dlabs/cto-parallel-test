# Cipher Shop Frontend (React + Vite + shadcn/ui)

Production-ready e-commerce frontend using React, Vite, Tailwind CSS, and shadcn/ui. Implements secure defaults with parameterized API endpoints and client-side input validation.

## Quick Start

- cp .env.example .env and set `VITE_API_BASE_URL` (e.g., `https://api.example.com`)
- npm ci
- npm start
- Open http://localhost:3000

## Features

- Routing: Home, Product List, Product Detail, Cart, Login, Register
- UI: Header with cart badge, Footer; shadcn/ui components (button, card, badge, input, form, navigation-menu)
- Styling: Tailwind CSS with shadcn/ui tokens (New York theme)
- Config: All endpoints come from `VITE_API_BASE_URL` (no hardcoded URLs)

## Security Notes

- Content Security Policy (CSP) set in `index.html` to restrict sources:
  - default-src 'self'
  - script-src 'self'
  - style-src 'self' 'unsafe-inline'
  - img-src 'self' data: blob: https:
  - font-src 'self' https: data:
  - connect-src 'self' https:
  - frame-ancestors 'none'
  - base-uri 'self'
  - form-action 'self'
- Additional meta headers: `Referrer-Policy: no-referrer`, `X-Content-Type-Options: nosniff`, `X-Frame-Options: DENY`
- Path handling: API URLs constructed with encoded path segments (`src/config.js`) to prevent traversal/injection.
- Route params validated (e.g., `ProductDetail.jsx` allows only safe IDs).
- No `dangerouslySetInnerHTML`; forms trim inputs; no credentials stored in localStorage.
- Dependency audit: `npm run audit:ci` fails on moderate/high/critical.

## Scripts

- `npm start` – Dev server on port 3000
- `npm run build` – Production build to `dist/`
- `npm run lint` – ESLint checks
- `npm run audit:ci` – Dependency audit (runtime deps only)

## Configuration

- Alias `@` → `src` configured in `vite.config.js` and `jsconfig.json`
- Tailwind config in `tailwind.config.js`; base styles in `src/index.css`

## Code Scanning

CodeQL and CI are configured in `.github/workflows`. Once the PR is open, check alerts:

```
gh api "/repos/5dlabs/cto-parallel-test/code-scanning/alerts?state=open&pr=<PR_NUMBER>"
```


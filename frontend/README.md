# Cipher Shop Frontend (React + Vite)

Secure, modern e-commerce frontend built with React, Vite, Tailwind CSS, and shadcn/ui.

## Quick Start

- Copy the environment file and set your API base URL:

  cp .env.example .env
  # Edit .env to point to your backend API
  # VITE_API_BASE_URL=https://api.example.com

- Install and run:

  npm ci
  npm run dev

Visit http://localhost:3000

## Security Defaults

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
- Additional headers via meta: `Referrer-Policy: no-referrer`, `X-Content-Type-Options: nosniff`, `X-Frame-Options: DENY`
- Path handling: API URLs are constructed with encoded path segments (`src/config.js`) to prevent traversal/injection.
- Route params validated (e.g., `ProductDetail.jsx` only allows safe IDs).
- No `dangerouslySetInnerHTML`; all user input trimmed and validated in forms.
- No secrets committed; use environment variables for configuration.

## Lint, Build, Audit

  npm run lint
  npm run build
  npm run audit:ci

Audit command checks prod dependencies only and fails for moderate+.

## Notes

- If your API runs on a different domain, the CSP’s `connect-src` allows `https:` by default. Narrow this to your exact origin in production via server headers for stronger security.
- The repo also contains a Next.js scaffold under `app/` that is not used by the Vite app. Keep them separate during development/build as needed.

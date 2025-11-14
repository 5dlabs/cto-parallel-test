## Implementation Summary

- Built React + Vite frontend with Tailwind CSS and shadcn/ui components.
- Pages: Home, Product List, Product Detail, Cart, Login, Register.
- Header/Footer and navigation menu with cart badge.
- API integration parameterized via environment (no hardcoded endpoints).

## Changes Made

- Added shadcn/ui components: button, card, badge, input, form, navigation-menu.
- Implemented routing and pages; cart context with localStorage persistence.
- Hardened security: strict CSP, safe id validation, URL encoding.
- Frontend CI workflow: lint, build, npm audit (threshold configurable).
- CodeQL workflow present.

## Testing Performed

- npm ci, npm run lint, npm run build.
- npm audit: 0 vulnerabilities (runtime).

## Notes

- Configure `VITE_API_BASE_URL` or `NEXT_PUBLIC_API_BASE_URL` for live API.
- Dev server port via `VITE_PORT`/`PORT`.
- Audit threshold via `AUDIT_LEVEL` (default: moderate).

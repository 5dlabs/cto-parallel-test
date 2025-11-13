Frontend (Vite + React + shadcn/ui)

Overview
- Stack: Vite, React 18, Tailwind CSS, shadcn/ui, Radix UI
- Routing: react-router-dom
- Security: basic CSP in index.html, no hardcoded secrets, API base via env, safe URL encoding

Run
- Env file: copy `frontend/.env.example` to `frontend/.env` and set `VITE_API_BASE_URL`
- Install: `cd frontend && pnpm install` (pnpm lock committed)
- Dev: `pnpm dev` (port from `PORT` env var; default 3000)
- Build: `pnpm build` outputs `frontend/dist`

Configuration
- `VITE_API_BASE_URL` should be a relative path like `/api` and served via a reverse proxy to your API. This keeps CSP strict and avoids cross-origin requests.
- `PORT` sets dev/preview port

Security Notes
- Never store credentials client-side. Login uses POST to `${VITE_API_BASE_URL}/auth/login` and assumes httpOnly cookies for session.
- Product routes encode IDs via `encodeURIComponent` to avoid injection in paths.
- CSP in `frontend/index.html` restricts sources during dev and production.

Pages
- Home, ProductList, ProductDetail, Cart, Login, Register

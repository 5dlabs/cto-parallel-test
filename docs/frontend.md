Frontend: E‑Commerce React App
================================

Overview
- Location: `frontend/`
- Stack: React + Vite, Tailwind CSS, shadcn/ui components
- Port: `3000` by default (override with `PORT` env)

Run Locally
- cd `frontend`
- `npm install`
- `npm start` then open http://localhost:3000

Configuration
- API base URL via `VITE_API_BASE_URL` (default `/api`): see `frontend/src/config.js`.

Routes
- `/` HomePage
- `/products` ProductList
- `/products/:id` ProductDetail
- `/cart` Cart
- `/login` Login
- `/register` Register

Security Notes
- No hardcoded secrets; all endpoints are env‑driven.
- Client validates inputs (zod + react‑hook‑form); never logs sensitive data.
- Dev tooling hardened: upgraded to `vite@7` to remove `esbuild` dev‑server vulnerability (moderate). `npm audit` reports 0 vulnerabilities.
 - Added a default Content Security Policy in `frontend/index.html` to mitigate XSS (`default-src 'self'`; `connect-src` allows `ws:` for Vite HMR only).
 - Locked Vite dev/preview to `127.0.0.1` with `strictPort: true` to avoid accidental network exposure and port hopping.

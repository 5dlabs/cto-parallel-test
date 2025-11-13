# E-Shop Frontend (Vite + React + shadcn/ui)

A modern e‑commerce frontend built with Vite, React 18, Tailwind CSS, and shadcn/ui.

## Features

- Modern UI with shadcn/ui components
- Mobile‑first responsive design (375px / 768px / 1920px)
- WCAG AA accessible (semantic HTML, keyboard nav, contrast)
- Client‑side validation with Zod + React Hook Form
- Environment‑driven API base URL (no hardcoded endpoints)

## Pages

- Home (`/`) – Landing page
- Products (`/products`) – Product grid
- Product Detail (`/products/:id`) – Single product view
- Cart (`/cart`) – Shopping cart
- Login (`/login`) – Auth form
- Register (`/register`) – Registration form

## Components

- Layout: `Header`, `Footer`
- UI (shadcn/ui): `Button`, `Card`, `Badge`, `Input`, `NavigationMenu`

## Getting Started

### Prerequisites

- Node.js 20+

### Install

```bash
npm install
```

### Development

```bash
VITE_API_BASE_URL=https://api.example.test npm start
# App runs on http://localhost:3000
```

### Build

```bash
npm run build
```

### Lint

```bash
npm run lint
```

## Configuration

- `VITE_API_BASE_URL` controls the backend API base (defaults to `/api`).
- Example: copy `.env.example` to `.env` and set `VITE_API_BASE_URL`.

## Security

- Strict Content Security Policy in `index.html` to mitigate XSS.
- No hardcoded secrets; configuration via env only.
- Avoids `dangerouslySetInnerHTML`; uses safe URL encoding.

## Project Structure (relevant parts)

```
frontend/
├── src/
│   ├── App.jsx
│   ├── main.jsx
│   ├── config.js           # exports CONFIG.apiBaseUrl from env
│   ├── components/
│   │   ├── Header.jsx
│   │   ├── Footer.jsx
│   │   └── ui/             # shadcn/ui components
│   └── pages/              # Home, Products, ProductDetail, Cart, Login, Register
├── index.html              # Vite entry with CSP
├── package.json            # scripts
└── tailwind.config.js
```

## Future Enhancements

- State management for cart and authentication
- API integration for products and orders
- Payment provider integration
- Search and filtering

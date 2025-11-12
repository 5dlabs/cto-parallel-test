# E-Shop Frontend

A modern e-commerce frontend application built with Next.js 15, React 19, TypeScript, Tailwind CSS, and shadcn/ui.

## Features

- 🎨 Modern UI with shadcn/ui components
- 📱 Mobile-first responsive design (375px/768px/1920px)
- ♿ WCAG AA accessible
- 🎯 TypeScript strict mode
- 🚀 Next.js 15 App Router
- 🎨 Tailwind CSS for styling
- 🔒 Client-side form validation

## Pages

- **Home** (`/`) - Landing page with hero section and features
- **Products** (`/products`) - Product listing with grid layout
- **Product Detail** (`/products/[id]`) - Individual product view
- **Cart** (`/cart`) - Shopping cart with order summary
- **Login** (`/login`) - User login form
- **Register** (`/register`) - User registration form

## Components

### Layout
- `Header` - Navigation header with cart badge and user menu
- `Footer` - Site footer with links

### UI Components (shadcn/ui)
- Button
- Card
- Badge
- Input
- Label
- Form
- Navigation Menu
- Separator

## Getting Started

### Prerequisites

- Node.js 22.x or later
- pnpm 10.x or later

### Installation

```bash
pnpm install
```

### Development

```bash
pnpm dev
```

Open [http://localhost:3000](http://localhost:3000) to view the application.

### Build

```bash
pnpm build
```

### Production

```bash
pnpm start
```

### Lint

```bash
pnpm lint
```

## Technology Stack

- **Next.js 16.0.2** - React framework with App Router
- **React 19.2.0** - UI library
- **TypeScript 5.9.3** - Type safety
- **Tailwind CSS 4.1.17** - Utility-first CSS
- **shadcn/ui** - High-quality React components
- **lucide-react** - Icon library

## Accessibility

This application follows WCAG AA standards:

- Semantic HTML elements
- ARIA labels and attributes
- Keyboard navigation support
- Proper color contrast
- Form validation with accessible error messages

## Responsive Design

The application is mobile-first and fully responsive:

- Mobile: 375px and up
- Tablet: 768px and up
- Desktop: 1920px and up

## Project Structure

```
frontend/
├── app/                  # Next.js app router pages
│   ├── cart/            # Shopping cart page
│   ├── login/           # Login page
│   ├── products/        # Products listing and detail pages
│   ├── register/        # Registration page
│   ├── layout.tsx       # Root layout with Header/Footer
│   └── page.tsx         # Home page
├── components/
│   ├── layout/          # Layout components
│   │   ├── Header.tsx
│   │   └── Footer.tsx
│   └── ui/              # shadcn/ui components
├── lib/
│   └── utils.ts         # Utility functions
└── public/              # Static assets
```

## Future Enhancements

- State management for cart and user authentication
- API integration for products and orders
- Payment gateway integration
- Product search and filtering
- User profile management
- Order history

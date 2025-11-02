# E-Commerce Frontend

A modern, production-ready e-commerce frontend built with Next.js 15, React 19, TypeScript, Tailwind CSS, and shadcn/ui.

## Tech Stack

- **Framework**: Next.js 15.5.6
- **UI Library**: React 19.2.0
- **Language**: TypeScript 5.9.3 (strict mode)
- **Styling**: Tailwind CSS 3.4.18
- **Components**: shadcn/ui with Radix UI primitives
- **Icons**: Lucide React
- **Package Manager**: pnpm

## Features

- 🎨 Modern, responsive UI with shadcn/ui components
- 📱 Mobile-first design (supports 375px, 768px, 1920px)
- ♿ WCAG AA accessible
- 🔍 SEO-friendly with Next.js metadata
- 🚀 Optimized production builds
- 🎯 TypeScript strict mode for type safety
- 🎭 Server and client components with Next.js App Router

## Pages

- **Home** (`/`) - Landing page with hero section and features
- **Products** (`/products`) - Product grid with filtering
- **Product Detail** (`/products/[id]`) - Individual product page
- **Cart** (`/cart`) - Shopping cart with order summary
- **Login** (`/login`) - User authentication
- **Register** (`/register`) - User registration

## Getting Started

### Prerequisites

- Node.js 22.20.0 or later
- pnpm 10.20.0 or later (recommended) or npm 11.6.2

### Installation

```bash
# Install dependencies
pnpm install
```

### Development

```bash
# Start development server
pnpm dev

# The app will be available at http://localhost:3000
```

### Production Build

```bash
# Create production build
pnpm build

# Start production server
pnpm start
```

### Linting

```bash
# Run ESLint
pnpm lint
```

## Project Structure

```
frontend/
├── app/                      # Next.js App Router pages
│   ├── cart/                # Cart page
│   ├── login/               # Login page
│   ├── products/            # Products pages
│   │   └── [id]/           # Dynamic product detail
│   ├── register/            # Register page
│   ├── layout.tsx           # Root layout
│   ├── page.tsx             # Home page
│   └── globals.css          # Global styles
├── components/              # React components
│   ├── ui/                  # shadcn/ui components
│   │   ├── badge.tsx
│   │   ├── button.tsx
│   │   ├── card.tsx
│   │   ├── input.tsx
│   │   ├── label.tsx
│   │   └── navigation-menu.tsx
│   ├── Header.tsx           # Header with navigation
│   └── Footer.tsx           # Footer component
├── lib/                     # Utility functions
│   └── utils.ts             # cn() utility for class merging
├── public/                  # Static assets
├── components.json          # shadcn/ui configuration
├── next.config.ts           # Next.js configuration
├── tailwind.config.ts       # Tailwind CSS configuration
├── tsconfig.json            # TypeScript configuration
└── package.json             # Project dependencies
```

## Component Library

This project uses [shadcn/ui](https://ui.shadcn.com/), which copies component source code into your project. This approach gives you:

- Full control over component styling
- No dependency on external UI libraries
- Type-safe components with TypeScript
- Customizable with Tailwind CSS

### Available Components

- **Button**: Primary actions and links
- **Card**: Content containers with header, content, and footer
- **Badge**: Status indicators and labels
- **Input**: Form inputs with validation
- **Label**: Form labels
- **Navigation Menu**: Responsive navigation with dropdown support

## Styling

The project uses Tailwind CSS with a custom theme based on shadcn/ui defaults:

- **Colors**: Slate-based color palette with light/dark mode support
- **Spacing**: Consistent spacing scale
- **Typography**: System font stack with proper hierarchy
- **Responsive**: Mobile-first breakpoints (sm, md, lg, xl, 2xl)

## Accessibility

All components follow WCAG AA guidelines:

- Semantic HTML structure
- ARIA labels for interactive elements
- Keyboard navigation support
- Focus indicators
- Screen reader friendly

## Performance

- Static generation for optimal performance
- Dynamic routes for product pages
- Optimized bundle sizes (~102-126 kB first load)
- Image optimization ready (using Next.js Image component recommended)
- CSS optimization with Tailwind

## Development Guidelines

1. Use TypeScript strict mode for all new code
2. Follow shadcn/ui component patterns
3. Maintain mobile-first responsive design
4. Ensure accessibility compliance
5. Test all routes before committing
6. Run lint and build checks before PR

## Future Enhancements

- Connect to backend API for live data
- Implement state management (Context API or Zustand)
- Add user authentication flow
- Integrate payment processing
- Add product search and filtering
- Implement wishlist functionality
- Add user profile and order history

## License

Copyright © 2025 E-Shop. All rights reserved.

# E-Commerce Frontend

A modern e-commerce frontend application built with Next.js 15, React 19, TypeScript, and shadcn/ui.

## Features

- **Modern Stack**: Next.js 15 with React 19 and TypeScript
- **Styling**: Tailwind CSS with shadcn/ui component library
- **Responsive Design**: Mobile-first design supporting 375px, 768px, and 1920px viewports
- **Accessibility**: WCAG AA compliant with proper ARIA labels and semantic HTML
- **Type Safety**: Full TypeScript strict mode implementation

## Pages

- **Home (`/`)**: Landing page with hero section and feature highlights
- **Products (`/products`)**: Product listing with grid layout
- **Product Detail (`/products/[id]`)**: Individual product view with specifications
- **Cart (`/cart`)**: Shopping cart with order summary
- **Login (`/login`)**: User authentication form
- **Register (`/register`)**: User registration form

## Components

### UI Components (shadcn/ui)
- Button
- Card
- Badge
- Input
- Label

### Layout Components
- Header: Navigation with cart badge and user icon
- Footer: Simple footer with links

## Getting Started

### Prerequisites
- Node.js 22+
- pnpm 10+

### Installation

```bash
# Install dependencies
pnpm install

# Run development server
pnpm dev

# Build for production
pnpm build

# Start production server
pnpm start

# Run linter
pnpm lint

# Type check
pnpm type-check
```

## Development

The application is structured following Next.js 15 App Router conventions:

```
frontend/
├── src/
│   ├── app/                 # App router pages
│   │   ├── layout.tsx       # Root layout
│   │   ├── page.tsx         # Home page
│   │   ├── products/        # Products pages
│   │   ├── cart/            # Cart page
│   │   ├── login/           # Login page
│   │   └── register/        # Register page
│   ├── components/          # React components
│   │   ├── ui/              # shadcn/ui components
│   │   ├── Header.tsx       # Header component
│   │   └── Footer.tsx       # Footer component
│   └── lib/                 # Utilities
│       └── utils.ts         # Helper functions
├── public/                  # Static assets
└── package.json
```

## Responsive Design

The application is fully responsive with breakpoints:
- Mobile: 375px - 767px
- Tablet: 768px - 1919px
- Desktop: 1920px+

## Accessibility

- Semantic HTML elements
- ARIA labels on interactive elements
- Keyboard navigation support
- Proper form labels and validation
- Focus management

## Future Enhancements

- Connect to backend API endpoints
- Implement state management (Context/Redux)
- Add cart functionality with local storage
- Implement authentication flow
- Add product search and filtering
- Image optimization and lazy loading
- PWA capabilities

## Tech Stack

- **Framework**: Next.js 15.5.6
- **React**: 19.2.0
- **TypeScript**: 5.9.3
- **Styling**: Tailwind CSS 3.4.18
- **UI Components**: shadcn/ui (class-variance-authority, clsx, tailwind-merge)
- **Icons**: lucide-react 0.460.0

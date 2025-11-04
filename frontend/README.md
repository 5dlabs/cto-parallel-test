# E-Commerce Frontend

A modern e-commerce frontend application built with Next.js 15, React 19, TypeScript, Tailwind CSS, and shadcn/ui.

## Tech Stack

- **Next.js 15** - React framework with App Router
- **React 19** - UI library
- **TypeScript** - Type-safe JavaScript
- **Tailwind CSS** - Utility-first CSS framework
- **shadcn/ui** - High-quality UI components
- **Lucide React** - Beautiful icon library

## Features

- 🏠 **Home Page** - Landing page with features and call-to-action
- 📦 **Product List** - Grid of products with categories and pricing
- 🔍 **Product Detail** - Detailed product view with reviews and features
- 🛒 **Shopping Cart** - Cart management with quantity controls
- 🔐 **Authentication** - Login and registration pages
- 📱 **Responsive Design** - Mobile-first design (375px/768px/1920px)
- ♿ **Accessible** - WCAG AA compliant

## Getting Started

### Prerequisites

- Node.js 18+ or 20+
- pnpm 8+ (recommended) or npm

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
```

### Development

```bash
# Run type checking
pnpm type-check

# Run linter
pnpm lint
```

## Project Structure

```
frontend/
├── src/
│   ├── app/              # Next.js App Router pages
│   │   ├── cart/         # Shopping cart page
│   │   ├── login/        # Login page
│   │   ├── products/     # Product list and detail pages
│   │   ├── register/     # Registration page
│   │   ├── layout.tsx    # Root layout
│   │   └── page.tsx      # Home page
│   ├── components/       # React components
│   │   ├── ui/          # shadcn/ui components
│   │   ├── Header.tsx   # Header navigation
│   │   └── Footer.tsx   # Footer component
│   └── lib/             # Utility functions
│       └── utils.ts     # Class name utility
├── public/              # Static assets
├── package.json         # Dependencies
├── tsconfig.json        # TypeScript config
├── tailwind.config.ts   # Tailwind CSS config
└── next.config.ts       # Next.js config
```

## Available Routes

- `/` - Home page
- `/products` - Product listing
- `/products/[id]` - Product detail
- `/cart` - Shopping cart
- `/login` - User login
- `/register` - User registration

## Component Library

All UI components are from shadcn/ui and fully customizable:

- **Button** - Various styles and sizes
- **Card** - Content containers
- **Badge** - Labels and tags
- **Input** - Form inputs
- **Label** - Form labels
- **Navigation Menu** - Header navigation

## Responsive Breakpoints

- **Mobile**: 375px - 767px
- **Tablet**: 768px - 1919px
- **Desktop**: 1920px+

## Accessibility

- Semantic HTML
- ARIA labels where needed
- Keyboard navigation support
- Focus indicators
- Color contrast compliance (WCAG AA)

## Future Enhancements

- Integration with backend API
- State management (Redux/Zustand)
- User authentication flow
- Payment integration
- Product search and filtering
- User reviews and ratings
- Order history
- Wishlist functionality

## License

MIT

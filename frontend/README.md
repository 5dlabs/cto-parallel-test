# E-Commerce Frontend

Modern e-commerce frontend application built with Next.js 15, React 19, TypeScript, and shadcn/ui.

## Tech Stack

- **Next.js 15** - React framework with App Router
- **React 19** - UI library
- **TypeScript** - Type safety
- **Tailwind CSS** - Utility-first CSS framework
- **shadcn/ui** - High-quality React components
- **Lucide React** - Icon library

## Features

- 🏠 **Home Page** - Landing page with hero section and features
- 🛍️ **Product List** - Grid layout with product cards
- 📦 **Product Detail** - Individual product pages with full details
- 🛒 **Shopping Cart** - Cart management with quantity controls
- 🔐 **Authentication** - Login and registration pages
- 📱 **Responsive Design** - Mobile-first approach (375px, 768px, 1920px)
- ♿ **Accessible** - WCAG AA compliant components

## Getting Started

### Prerequisites

- Node.js 18+ 
- pnpm 8+

### Installation

```bash
# Install dependencies
pnpm install
```

### Development

```bash
# Run development server
pnpm dev
```

Open [http://localhost:3000](http://localhost:3000) to view the application.

### Build

```bash
# Type check
pnpm type-check

# Lint
pnpm lint

# Build for production
pnpm build

# Start production server
pnpm start
```

## Project Structure

```
src/
├── app/                    # Next.js app directory
│   ├── cart/              # Shopping cart page
│   ├── login/             # Login page
│   ├── products/          # Products listing and detail
│   ├── register/          # Registration page
│   ├── layout.tsx         # Root layout
│   ├── page.tsx           # Home page
│   └── globals.css        # Global styles
├── components/            # React components
│   ├── ui/               # shadcn/ui components
│   ├── Header.tsx        # Header navigation
│   └── Footer.tsx        # Footer
└── lib/                   # Utility functions
    └── utils.ts          # Helper utilities
```

## Routes

- `/` - Home page
- `/products` - Product listing
- `/products/[id]` - Product detail
- `/cart` - Shopping cart
- `/login` - Login
- `/register` - Registration

## Components

### shadcn/ui Components

The following shadcn/ui components are included:

- Button
- Card
- Badge
- Input
- Label
- Navigation Menu

### Custom Components

- **Header** - Navigation bar with cart badge and user menu
- **Footer** - Site footer with links

## Styling

The application uses:

- **Tailwind CSS** for utility classes
- **CSS Variables** for theming
- **shadcn/ui** design tokens for consistent styling

## Responsive Breakpoints

- Mobile: 375px
- Tablet: 768px
- Desktop: 1920px

## Future Enhancements

- Connect to backend API
- State management (e.g., Zustand, React Context)
- Real-time cart updates
- Product search and filtering
- User profile pages
- Order history
- Payment integration

## License

MIT

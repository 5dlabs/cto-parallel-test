# E-commerce Frontend

A modern React-based e-commerce frontend built with React 18.2.0, React Router 6.14.2, Tailwind CSS, and shadcn/ui components.

## Features

- 🛍️ Product catalog with grid layout
- 🛒 Shopping cart functionality
- 👤 User authentication (Login/Register)
- 📱 Responsive design for mobile and desktop
- 🎨 Modern UI with shadcn/ui components
- 🚀 Fast navigation with React Router

## Tech Stack

- **React** 18.2.0 - UI library
- **React Router** 6.14.2 - Client-side routing
- **Tailwind CSS** - Utility-first CSS framework
- **shadcn/ui** - High-quality React components built on Radix UI
- **Lucide React** - Icon library
- **Axios** - HTTP client for API calls

## Getting Started

### Prerequisites

- Node.js 14+ and npm

### Installation

```bash
# Install dependencies
npm install

# Start development server
npm start

# Build for production
npm run build

# Run tests
npm test
```

## Available Routes

- `/` - Home page with hero section and features
- `/products` - Product listing page
- `/products/:id` - Individual product detail page
- `/cart` - Shopping cart
- `/login` - User login
- `/register` - User registration

## Project Structure

```
frontend/
├── public/              # Static files
├── src/
│   ├── components/      # React components
│   │   ├── ui/         # shadcn/ui components
│   │   ├── Header.jsx  # Navigation header
│   │   ├── Footer.jsx  # Footer component
│   │   ├── HomePage.jsx
│   │   ├── ProductList.jsx
│   │   ├── ProductDetail.jsx
│   │   ├── Cart.jsx
│   │   ├── Login.jsx
│   │   └── Register.jsx
│   ├── lib/            # Utility functions
│   ├── App.js          # Main app component with routing
│   ├── index.js        # Entry point
│   └── index.css       # Global styles with Tailwind
├── package.json
└── tailwind.config.js
```

## Component Overview

### Header
- Responsive navigation bar
- Cart icon with item count badge
- Mobile menu support

### Footer
- Copyright information
- Quick links (Privacy Policy, Terms, Contact)

### HomePage
- Hero section with call-to-action
- Feature cards showcasing benefits
- Responsive grid layout

### ProductList
- Grid of product cards
- Product images, prices, and descriptions
- "Add to Cart" and "View Details" buttons
- Low stock indicators

### ProductDetail
- Detailed product view with large image
- Quantity selector
- Product features list
- Shipping and payment information

### Cart
- List of cart items with images
- Quantity adjustments
- Remove item functionality
- Order summary with subtotal, tax, and total
- Empty cart state

### Login/Register
- Form validation
- Error handling
- Responsive design
- Navigation between login and register

## Styling

This project uses Tailwind CSS with custom theme configuration:

- Primary color: Blue (customizable via CSS variables)
- Responsive breakpoints: sm, md, lg, xl, 2xl
- Dark mode support (configured but not enabled by default)

## Future Enhancements

- API integration with backend
- Global state management (Redux/Context)
- User authentication persistence
- Product search and filtering
- Wishlist functionality
- Order history
- Payment integration
- Real-time inventory updates

## License

MIT

# E-Commerce Frontend

React-based frontend for the e-commerce application, built with shadcn/ui and Tailwind CSS.

## Features

- Modern UI with shadcn/ui components
- Responsive design with Tailwind CSS
- React Router for navigation
- Product browsing and filtering
- Shopping cart functionality
- User authentication (login/register)

## Getting Started

### Prerequisites

- Node.js 14 or higher
- npm or yarn

### Installation

```bash
npm install
```

Note: If you encounter issues with devDependencies not installing, ensure NODE_ENV is not set to 'production':

```bash
export NODE_ENV=development
npm install
```

### Running the Development Server

```bash
npm start
```

The app will open at [http://localhost:3000](http://localhost:3000)

### Building for Production

```bash
npm run build
```

### Running Tests

```bash
npm test
```

## Project Structure

```
src/
├── components/
│   ├── ui/              # shadcn/ui components
│   │   ├── button.jsx
│   │   ├── card.jsx
│   │   ├── input.jsx
│   │   ├── label.jsx
│   │   └── badge.jsx
│   ├── Header.jsx       # Navigation header
│   ├── Footer.jsx       # Page footer
│   ├── HomePage.jsx     # Landing page
│   ├── ProductList.jsx  # Product catalog
│   ├── ProductDetail.jsx # Single product view
│   ├── Cart.jsx         # Shopping cart
│   ├── Login.jsx        # Login form
│   └── Register.jsx     # Registration form
├── lib/
│   └── utils.js         # Utility functions
├── App.js               # Main app with routing
├── index.js             # Entry point
└── index.css            # Global styles

## Routes

- `/` - Home page
- `/products` - Product list
- `/products/:id` - Product detail
- `/cart` - Shopping cart
- `/login` - User login
- `/register` - User registration

## Technologies

- React 18.2.0
- React Router DOM 6.14.2
- shadcn/ui (Radix UI components)
- Tailwind CSS 3.3.3
- Axios 1.4.0
- Lucide React (icons)

## Development Notes

This frontend is designed to work with the Rust backend API. Currently, it uses mock data for demonstration purposes. API integration will be added in future updates.

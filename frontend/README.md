# E-Commerce Frontend

A React-based e-commerce frontend built with shadcn/ui and Tailwind CSS.

## Features

- **React 18.2.0**: Modern React with hooks
- **React Router 6.14.2**: Client-side routing
- **shadcn/ui**: High-quality UI components built on Radix UI
- **Tailwind CSS**: Utility-first CSS framework
- **Responsive Design**: Mobile-first responsive layouts

## Components

- **Header**: Navigation with cart badge
- **Footer**: Simple footer
- **HomePage**: Landing page with features
- **ProductList**: Grid of product cards
- **ProductDetail**: Single product view
- **Cart**: Shopping cart with order summary
- **Login**: User login form
- **Register**: User registration form

## Installation

```bash
npm install
```

## Available Scripts

### Development Server

```bash
npm start
```

Runs the app in development mode at [http://localhost:3000](http://localhost:3000).

### Production Build

```bash
npm run build
```

Creates an optimized production build in the `build/` folder.

### Run Tests

```bash
npm test
```

Launches the test runner.

## Project Structure

```
frontend/
├── public/
│   └── index.html
├── src/
│   ├── components/
│   │   ├── ui/           # shadcn/ui components
│   │   ├── Header.jsx
│   │   ├── Footer.jsx
│   │   ├── HomePage.jsx
│   │   ├── ProductList.jsx
│   │   ├── ProductDetail.jsx
│   │   ├── Cart.jsx
│   │   ├── Login.jsx
│   │   └── Register.jsx
│   ├── lib/
│   │   └── utils.js      # Utility functions
│   ├── App.js
│   ├── index.js
│   └── index.css
├── package.json
├── tailwind.config.js
└── postcss.config.js
```

## Routing

- `/` - Home page
- `/products` - Product list
- `/products/:id` - Product detail
- `/cart` - Shopping cart
- `/login` - Login page
- `/register` - Registration page

## Styling

The app uses Tailwind CSS with shadcn/ui's design system. CSS variables are defined in `index.css` for theming support.

## Future Enhancements

- Connect to backend API
- Add authentication context
- Implement cart state management
- Add form validation
- Add error boundaries
- Add loading states
- Implement real-time inventory updates

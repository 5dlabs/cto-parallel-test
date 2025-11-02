# E-Commerce Frontend

A React-based e-commerce frontend application built with Material-UI.

## Tech Stack

- **React** 18.2.0
- **Material-UI** 5.14.0
- **React Router DOM** 6.14.2
- **Axios** 1.4.0
- **Webpack** 5.88.2

## Features

### Components

1. **Header** - Navigation bar with cart badge and mobile-responsive drawer
2. **Footer** - Simple copyright footer
3. **HomePage** - Landing page with hero section and feature cards
4. **ProductList** - Grid of product cards with add-to-cart functionality
5. **ProductDetail** - Individual product view with quantity selector
6. **Cart** - Shopping cart with order summary and calculations
7. **Login** - User login form with validation
8. **Register** - User registration form with validation

### Routes

- `/` - Home page
- `/products` - Product list page
- `/products/:id` - Product detail page
- `/cart` - Shopping cart
- `/login` - User login
- `/register` - User registration

## Installation

```bash
npm install
```

Note: Use `npm install --include=dev` if dev dependencies are not installed automatically.

## Development

Start the development server:

```bash
npm start
```

The application will open automatically in your browser at `http://localhost:3000`.

## Production Build

Build the application for production:

```bash
npm run build
```

The production-ready files will be in the `dist/` directory.

## Project Structure

```
frontend/
├── public/
│   └── index.html          # HTML template
├── src/
│   ├── components/         # React components
│   │   ├── Header.js
│   │   ├── Footer.js
│   │   ├── HomePage.js
│   │   ├── ProductList.js
│   │   ├── ProductDetail.js
│   │   ├── Cart.js
│   │   ├── Login.js
│   │   └── Register.js
│   ├── App.js              # Main app component with routing
│   └── index.js            # Application entry point
├── webpack.config.js       # Webpack configuration
└── package.json            # Dependencies and scripts
```

## Features

### Responsive Design
- Mobile-first approach
- Responsive grid layouts
- Mobile navigation drawer for small screens

### Material-UI Theme
- Primary color: #1976d2 (blue)
- Secondary color: #dc004e (pink)
- Consistent typography using Roboto font

### Mock Data
- Product list uses mock data for demonstration
- Cart functionality uses localStorage
- Ready for backend API integration

## Future Enhancements

- Connect to backend API endpoints
- Add user authentication with JWT
- Implement real cart functionality with backend
- Add product search and filtering
- Add checkout flow
- Add order history

## License

Copyright © 2024 E-Commerce Store

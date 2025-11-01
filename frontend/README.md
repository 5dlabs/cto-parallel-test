# E-Commerce Frontend

A modern React-based frontend application for e-commerce, built with Material-UI.

## Features

- **Responsive Design**: Works seamlessly on mobile, tablet, and desktop devices
- **Modern UI**: Built with Material-UI (MUI) component library
- **Client-Side Routing**: React Router DOM for smooth navigation
- **Shopping Cart**: Persistent cart stored in localStorage
- **User Authentication**: Login and registration forms (ready for backend integration)
- **Product Catalog**: Browse products with search functionality

## Components

### Layout Components
- **Header**: Navigation bar with cart badge and authentication links
- **Footer**: Simple footer with copyright information

### Page Components
- **HomePage**: Landing page with call-to-action and feature highlights
- **ProductList**: Grid view of all products with search functionality
- **ProductDetail**: Detailed view of individual products
- **Cart**: Shopping cart with quantity management and checkout
- **Login**: User login form
- **Register**: New user registration form

## Getting Started

### Prerequisites
- Node.js (v14 or higher)
- npm or yarn

### Installation

```bash
# Install dependencies
npm install
```

### Available Scripts

#### `npm start`
Runs the app in development mode.
Open [http://localhost:3000](http://localhost:3000) to view it in your browser.

The page will reload when you make changes.

#### `npm test`
Launches the test runner in interactive watch mode.

#### `npm run build`
Builds the app for production to the `build` folder.
It correctly bundles React in production mode and optimizes the build for the best performance.

#### `npm run eject`
**Note: this is a one-way operation. Once you `eject`, you can't go back!**

If you need to customize the build configuration, you can eject at any time.

## Project Structure

```
frontend/
├── public/
│   ├── index.html          # HTML template
│   └── robots.txt          # SEO configuration
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
│   ├── App.js              # Main application component
│   ├── App.test.js         # App tests
│   ├── index.js            # Application entry point
│   ├── index.css           # Global styles
│   ├── setupTests.js       # Test configuration
│   └── reportWebVitals.js  # Performance monitoring
├── package.json            # Dependencies and scripts
└── README.md               # This file
```

## Technologies Used

- **React** (18.2.0): UI library
- **React Router DOM** (6.14.2): Client-side routing
- **Material-UI** (5.14.0): UI component library
- **Axios** (1.4.0): HTTP client (ready for API integration)
- **Emotion**: CSS-in-JS styling

## Routes

- `/` - Home page
- `/products` - Product listing
- `/products/:id` - Product detail page
- `/cart` - Shopping cart
- `/login` - User login
- `/register` - User registration

## State Management

Currently, the application uses:
- **localStorage** for cart persistence
- **localStorage** for authentication tokens
- Component-level state with React hooks

For production, consider implementing:
- Context API for global state
- Redux or Zustand for complex state management
- React Query for server state

## Backend Integration

The application is structured for easy backend integration:

1. **API Configuration**: Update the base URL in axios configuration
2. **Authentication**: JWT token handling is already implemented
3. **Cart API**: Replace localStorage with backend API calls
4. **Product API**: Connect to real product endpoints

Example API integration:

```javascript
import axios from 'axios';

const API_BASE_URL = process.env.REACT_APP_API_URL || 'http://localhost:8080/api';

// Configure axios
const api = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

// Add auth token to requests
api.interceptors.request.use((config) => {
  const token = localStorage.getItem('token');
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

export default api;
```

## Responsive Design

The application is fully responsive with breakpoints:
- **xs**: < 600px (mobile)
- **sm**: 600px - 960px (tablet)
- **md**: 960px - 1280px (small desktop)
- **lg**: 1280px+ (large desktop)

## Testing

The project includes a basic test setup with React Testing Library.

Run tests with:
```bash
npm test
```

For coverage:
```bash
npm test -- --coverage
```

## Future Enhancements

- [ ] Redux for state management
- [ ] React Query for API caching
- [ ] Product image uploads
- [ ] User profile management
- [ ] Order history
- [ ] Payment integration
- [ ] Product reviews and ratings
- [ ] Advanced filtering and sorting
- [ ] Wishlist functionality
- [ ] Social authentication (OAuth)

## Contributing

1. Create a feature branch
2. Make your changes
3. Run tests: `npm test`
4. Build: `npm run build`
5. Submit a pull request

## License

This project is part of the e-commerce application task implementation.

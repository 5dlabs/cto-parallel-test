# E-Commerce Frontend

React-based frontend application for the e-commerce platform with Material-UI components.

## Features

- **Responsive Design**: Mobile-first approach using Material-UI
- **User Authentication**: Login and registration with JWT
- **Product Catalog**: Browse and search products
- **Shopping Cart**: Add/remove items with quantity controls
- **Product Details**: View detailed product information
- **Navigation**: React Router for seamless SPA experience

## Tech Stack

- React 18.2.0
- React Router DOM 6.14.2
- Material-UI (MUI) 5.14.0
- Axios 1.4.0
- Emotion (CSS-in-JS)

## Prerequisites

- Node.js 14+ and npm

## Installation

```bash
npm install
```

## Configuration

Create a `.env` file based on `.env.example`:

```bash
cp .env.example .env
```

Edit `.env` to configure your backend API URL:

```
REACT_APP_API_URL=http://localhost:8080/api
```

## Running the Application

### Development Mode

```bash
npm start
```

The application will open at [http://localhost:3000](http://localhost:3000).

### Production Build

```bash
npm run build
```

Builds the app for production to the `build` folder.

### Testing

```bash
npm test
```

## Project Structure

```
frontend/
├── public/
│   └── index.html          # HTML template
├── src/
│   ├── components/         # React components
│   │   ├── Header.js       # Navigation header with cart
│   │   ├── Footer.js       # Application footer
│   │   ├── HomePage.js     # Landing page
│   │   ├── ProductList.js  # Product grid with search
│   │   ├── ProductDetail.js # Individual product view
│   │   ├── Cart.js         # Shopping cart
│   │   ├── Login.js        # Login form
│   │   └── Register.js     # Registration form
│   ├── App.js              # Main app with routing
│   └── index.js            # Entry point
├── package.json            # Dependencies
└── README.md              # This file
```

## Available Routes

- `/` - Home page
- `/products` - Product listing
- `/products/:id` - Product details
- `/cart` - Shopping cart
- `/login` - User login
- `/register` - User registration

## Features by Component

### Header
- Responsive navigation with mobile menu
- Cart icon with badge showing item count
- Links to all major sections

### HomePage
- Hero section with call-to-action
- Feature highlights
- Quick access to products and registration

### ProductList
- Grid layout with responsive columns
- Search functionality
- Add to cart buttons
- Stock status indicators

### ProductDetail
- Large product image
- Detailed description
- Quantity selector
- Add to cart functionality
- Stock availability

### Cart
- List of cart items with images
- Quantity controls
- Remove item functionality
- Order summary with totals
- Shipping and tax calculations
- Clear cart option

### Login/Register
- Form validation
- Password visibility toggle
- Error handling
- Links between forms

## API Integration

The frontend is designed to integrate with a REST API. When the backend is not available, it uses placeholder data for development purposes.

### Environment Variables

- `REACT_APP_API_URL`: Backend API base URL (default: `http://localhost:8080/api`)

### API Endpoints Expected

- `GET /products` - List all products
- `GET /products/:id` - Get product details
- `POST /auth/login` - User login
- `POST /auth/register` - User registration
- `GET /cart` - Get user's cart
- `POST /cart/add` - Add item to cart
- `DELETE /cart/remove/:id` - Remove item from cart
- `POST /cart/clear` - Clear cart

## Authentication

The application uses JWT tokens stored in localStorage:
- `token` - JWT authentication token
- `user_id` - User ID
- `username` - Username

Protected routes require authentication and redirect to login if not authenticated.

## Browser Support

- Chrome (latest)
- Firefox (latest)
- Safari (latest)
- Edge (latest)

## License

MIT

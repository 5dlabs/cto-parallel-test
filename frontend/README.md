# E-Commerce Frontend Application

This is a React-based frontend for the e-commerce test application.

## Structure

```
frontend/
├── package.json           # Project dependencies and scripts
├── public/
│   └── index.html        # HTML template
├── src/
│   ├── App.js            # Main application with routing and theme
│   ├── index.js          # Entry point
│   └── components/       # React components
│       ├── Header.js     # Navigation header
│       ├── Footer.js     # Page footer
│       ├── HomePage.js   # Landing page
│       ├── ProductList.js    # Product catalog
│       ├── ProductDetail.js  # Product detail page (placeholder)
│       ├── Cart.js       # Shopping cart (placeholder)
│       ├── Login.js      # Login form (placeholder)
│       └── Register.js   # Registration form (placeholder)
```

## Technologies

- **React** 18.2.0 - UI library
- **React Router DOM** 6.14.2 - Client-side routing
- **Material-UI** 5.14.0 - Component library
- **Axios** 1.13.1 - HTTP client (for future API integration)
- **Emotion** - CSS-in-JS styling (required by MUI)

## Routes

- `/` - Home page with welcome message
- `/products` - Product catalog (3 sample products)
- `/products/:id` - Product detail page
- `/cart` - Shopping cart
- `/login` - Login form
- `/register` - Registration form

## Setup Instructions

### Prerequisites

- Node.js 16.x or 18.x (recommended for react-scripts 5.0.1)
- npm 8.x or higher

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

### Note on Node.js Version

This project uses `react-scripts` 5.0.1, which works best with Node.js 16.x or 18.x. If you're using Node.js 22.x or higher, you may need to:

1. Use a Node version manager (nvm) to switch to Node 18:
   ```bash
   nvm install 18
   nvm use 18
   npm install
   npm start
   ```

2. Or use alternative build tools like Vite for Node 22 compatibility

## Features

### Current Implementation

- ✅ React Router setup with all routes configured
- ✅ Material-UI theme with custom colors (blue/pink)
- ✅ Responsive layout with Header and Footer
- ✅ Navigation between all pages
- ✅ Product list with sample data
- ✅ Form components for login and registration

### Placeholder Components

The following components are placeholder implementations and will be connected to backend APIs in future tasks:

- **Authentication**: Login and Register forms (no backend connection)
- **Cart State**: Cart displays placeholder message (no state management)
- **Product Data**: ProductList uses hardcoded sample products
- **Product Details**: ProductDetail shows placeholder content

## Future Enhancements

### Phase 1: State Management
- Add React Context or Redux for global state
- Implement authentication state management
- Create shopping cart state management

### Phase 2: API Integration
- Connect ProductList to GET /api/products
- Connect ProductDetail to GET /api/products/:id
- Connect Login to POST /api/auth/login
- Connect Register to POST /api/auth/register
- Connect Cart to backend cart API

### Phase 3: Enhanced Features
- Add error handling and loading states
- Implement form validation
- Add shopping cart functionality
- Implement user session management
- Add product search and filtering
- Add pagination for product list

## Development Guidelines

### Adding New Components

Create new components in the `src/components/` directory:

```javascript
import React from 'react';
import { Container, Typography } from '@mui/material';

function NewComponent() {
  return (
    <Container>
      <Typography variant="h4">New Component</Typography>
    </Container>
  );
}

export default NewComponent;
```

### Adding New Routes

Add routes in `src/App.js`:

```javascript
<Route path="/new-route" element={<NewComponent />} />
```

### Styling

Use Material-UI's `sx` prop for component styling:

```javascript
<Box sx={{ padding: 2, backgroundColor: 'primary.main' }}>
  Content
</Box>
```

## Testing

### Manual Testing Checklist

- [ ] Navigate to home page (/)
- [ ] Click "Shop Now" button → should go to /products
- [ ] View product list with 3 products
- [ ] Click "View Details" → should go to /products/:id
- [ ] Navigate to cart via header icon
- [ ] Navigate to login via header button
- [ ] Click "Sign Up" link → should go to /register
- [ ] All Material-UI components render correctly
- [ ] Theme colors (blue/pink) are applied
- [ ] Layout is responsive on mobile/tablet/desktop

## Architecture

### Component Hierarchy

```
App (Router + Theme)
├── Header (Navigation)
├── Routes
│   ├── HomePage
│   ├── ProductList
│   ├── ProductDetail
│   ├── Cart
│   ├── Login
│   └── Register
└── Footer
```

### Data Flow (Future)

```
User Action → Component → API Call → State Update → UI Update
```

## Troubleshooting

### Common Issues

**Issue**: `react-scripts: command not found`
- **Solution**: Ensure you're using Node.js 16.x or 18.x, then run `npm install`

**Issue**: Port 3000 already in use
- **Solution**: Use `PORT=3001 npm start` or kill the process using port 3000

**Issue**: Module not found errors
- **Solution**: Delete `node_modules` and `package-lock.json`, then run `npm install`

## License

This is a test application for educational purposes.

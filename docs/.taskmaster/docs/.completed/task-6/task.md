# Task 6: Frontend Components

## Overview
Create basic frontend structure and components for the application using React. This is a Level 0 task (no dependencies) that establishes the UI foundation with routing, layout components, and placeholder pages for the e-commerce test application.

## Context
This task provides the client-side interface for the API built in Tasks 1-5. It demonstrates that frontend and backend development can proceed in parallel when interfaces are well-defined. The frontend will eventually consume the REST APIs but can be developed with placeholder data initially.

## Objectives
1. Set up React application with routing
2. Create Material-UI themed layout components (Header, Footer)
3. Implement placeholder pages for all routes
4. Configure navigation between pages
5. Establish component structure for future API integration

## Dependencies
**None** - This is a Level 0 task that can run independently in parallel with Tasks 1, 3, and 4.

## Files to Create
- `frontend/package.json` - Node.js dependencies and scripts
- `frontend/src/App.js` - Main application with routing
- `frontend/src/components/Header.js` - Navigation header
- `frontend/src/components/Footer.js` - Page footer
- `frontend/src/components/HomePage.js` - Landing page
- `frontend/src/components/ProductList.js` - Product catalog page
- `frontend/src/components/ProductDetail.js` - Product detail page (placeholder)
- `frontend/src/components/Cart.js` - Shopping cart page (placeholder)
- `frontend/src/components/Login.js` - Login page (placeholder)
- `frontend/src/components/Register.js` - Registration page (placeholder)

## Technical Specifications

### Technology Stack
- **Framework**: React 18.2.0
- **Routing**: React Router DOM 6.14.2
- **UI Library**: Material-UI (MUI) 5.14.0
- **HTTP Client**: Axios 1.4.0 (for future API calls)
- **Styling**: Emotion (MUI's styling solution)
- **Build Tool**: React Scripts 5.0.1 (Create React App)

### Route Structure
```
/ → HomePage (landing page)
/products → ProductList (browse products)
/products/:id → ProductDetail (product details)
/cart → Cart (shopping cart)
/login → Login (authentication)
/register → Register (user registration)
```

### Component Hierarchy
```
App (Router, Theme Provider)
├── Header (Navigation, Cart Badge)
├── Routes
│   ├── HomePage
│   ├── ProductList
│   ├── ProductDetail
│   ├── Cart
│   ├── Login
│   └── Register
└── Footer
```

## Implementation Plan

### Step 1: Create package.json
Define project dependencies and scripts:

**Dependencies**:
- React core libraries (react, react-dom)
- React Router for navigation
- Material-UI for components
- Axios for API calls (future use)
- Emotion for styling (required by MUI)

**Scripts**:
- `start` - Development server (port 3000)
- `build` - Production build
- `test` - Jest test runner
- `eject` - Eject from CRA (not recommended)

**Configuration**:
- ESLint extends react-app
- Browserslist for target browsers

### Step 2: Create Main Application (frontend/src/App.js)
Set up routing and theming:

```jsx
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import { ThemeProvider, createTheme } from '@mui/material/styles';
import CssBaseline from '@mui/material/CssBaseline';

// Import all components

const theme = createTheme({
  palette: {
    primary: { main: '#1976d2' },    // Blue
    secondary: { main: '#dc004e' },  // Pink
  },
});

function App() {
  return (
    <ThemeProvider theme={theme}>
      <CssBaseline />  {/* Normalize CSS */}
      <Router>
        <Header />
        <main style={{ padding: '20px', minHeight: 'calc(100vh - 130px)' }}>
          <Routes>
            {/* Route definitions */}
          </Routes>
        </main>
        <Footer />
      </Router>
    </ThemeProvider>
  );
}
```

**Design Decisions**:
- `CssBaseline` normalizes CSS across browsers
- Theme defines consistent color palette
- Main element has padding and min-height for layout
- Router wraps entire app for navigation

### Step 3: Create Header Component
Navigation bar with cart badge:

**Features**:
- App title/logo linking to home
- Products navigation link
- Cart icon with badge (count placeholder)
- Login/Logout button (authentication state placeholder)
- Material-UI AppBar and Toolbar
- ShoppingCartIcon with Badge for cart items

**State Placeholder**:
```jsx
const isLoggedIn = false;        // TODO: Connect to auth context
const cartItemCount = 0;         // TODO: Connect to cart state
```

### Step 4: Create Footer Component
Simple copyright footer:

**Features**:
- Centered content in Container
- Current year (dynamic)
- Minimal styling
- Uses Material-UI Box for layout

### Step 5: Create HomePage Component
Landing page with call-to-action:

**Features**:
- Hero section with welcome message
- "Shop Now" button linking to products
- Centered layout with Material-UI Container
- Typography components for headings

### Step 6: Create ProductList Component
Product catalog with placeholder data:

**Features**:
- Grid layout (3 columns on desktop)
- Product cards with:
  - Placeholder image (grey box)
  - Product name and price
  - Description
  - "View Details" and "Add to Cart" buttons
- Hardcoded product array (3 sample products)
- Material-UI Grid, Card, CardContent, CardMedia

**Placeholder Data**:
```jsx
const products = [
  { id: 1, name: 'Product 1', price: 19.99, description: '...' },
  { id: 2, name: 'Product 2', price: 29.99, description: '...' },
  { id: 3, name: 'Product 3', price: 39.99, description: '...' },
];
```

Future: Replace with API call to backend

### Step 7: Create Placeholder Components
Simple placeholders for remaining routes:

**ProductDetail** (frontend/src/components/ProductDetail.js):
- Extract product ID from route params
- Display "Product Detail - ID: {id}"
- Placeholder for full product information
- Future: Fetch product details from API

**Cart** (frontend/src/components/Cart.js):
- Display "Shopping Cart"
- Placeholder for cart items list
- Placeholder for cart total
- Future: Display items from cart API

**Login** (frontend/src/components/Login.js):
- Login form with email and password fields
- Submit button (no functionality)
- Link to registration page
- Future: Connect to authentication API

**Register** (frontend/src/components/Register.js):
- Registration form fields
- Submit button (no functionality)
- Link to login page
- Future: Connect to user registration API

All placeholders use Material-UI components for consistency.

### Step 8: Directory Structure
```
frontend/
├── package.json
├── public/
│   └── index.html (Create React App default)
├── src/
│   ├── App.js
│   ├── index.js (Create React App default)
│   └── components/
│       ├── Header.js
│       ├── Footer.js
│       ├── HomePage.js
│       ├── ProductList.js
│       ├── ProductDetail.js
│       ├── Cart.js
│       ├── Login.js
│       └── Register.js
```

## Architectural Considerations

### Component-Based Architecture
- Each page is a separate component
- Shared layout (Header/Footer) wraps all routes
- Components are self-contained and reusable

### State Management
**Current**: Local component state and props
**Future**: Consider React Context or Redux for:
- User authentication state
- Shopping cart state
- Global application state

### Styling Strategy
Material-UI provides:
- Consistent design language
- Responsive components
- Theme customization
- Built-in accessibility

### Routing
React Router v6 features:
- Declarative route configuration
- Nested routes support
- URL parameter extraction
- Link component for navigation

### Placeholder Pattern
Components render static content initially:
- **Benefit**: UI development proceeds independently
- **Benefit**: Defines API contract (what data is needed)
- **Future**: Replace placeholders with API calls using Axios

## Future API Integration Points

1. **Authentication**:
   - Login form → POST /api/auth/login
   - Register form → POST /api/auth/register
   - Store JWT token in localStorage
   - Add Authorization header to requests

2. **Products**:
   - ProductList → GET /api/products
   - ProductDetail → GET /api/products/:id

3. **Cart**:
   - Cart component → GET /api/cart
   - Add to cart → POST /api/cart/add
   - Remove from cart → DELETE /api/cart/remove/:id

4. **State Management**:
   - Create AuthContext for user state
   - Create CartContext for cart state
   - Use React Context API or state management library

## Testing Strategy
See `acceptance-criteria.md` for detailed validation steps.

## Success Criteria
- All frontend files created
- Application starts without errors
- All routes are accessible
- Navigation works between pages
- Components render correctly
- Responsive layout on different screen sizes
- Material-UI theme applied consistently

## Related Tasks
- **Independent of** all backend tasks (runs in parallel)
- **Task 7**: Integration Tests (will eventually test frontend-backend integration)

## Risks and Considerations

1. **No Backend Integration**: Components use placeholder data. Integration with backend APIs is future work.

2. **No State Management**: Authentication and cart state are not persisted or shared between components.

3. **No Error Handling**: No error boundaries or API error handling.

4. **Basic Styling**: Minimal custom styling, relying on Material-UI defaults.

5. **No Testing**: No unit tests or component tests included.

## References
- [React Documentation](https://react.dev/)
- [React Router Documentation](https://reactrouter.com/)
- [Material-UI Documentation](https://mui.com/)
- [Create React App](https://create-react-app.dev/)
- Project PRD: `.taskmaster/docs/prd.txt`

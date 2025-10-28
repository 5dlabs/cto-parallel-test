# Task 6: Frontend Components

## Overview
Create basic frontend structure and components for the application using React, Material-UI, and React Router. This is a Level 0 task that has no dependencies and should execute in parallel with other Level 0 tasks (Tasks 1, 3, and 4).

## Context
This task is part of the parallel task execution test project. It establishes the frontend foundation that Task 7 (Integration Tests) will depend on. The implementation uses React 18 with Material-UI for styling and React Router for navigation, creating a placeholder e-commerce user interface.

## Objectives
1. Create `frontend/package.json` with React and Material-UI dependencies
2. Create `frontend/src/App.js` as the main application component with routing
3. Create basic UI components: Header, Footer, HomePage, ProductList, ProductDetail, Cart, Login, Register
4. Set up Material-UI theming and consistent design
5. Implement React Router for client-side navigation

## Dependencies
**None** - This is a Level 0 task that can run independently.

**Depended Upon By:**
- **Task 7 (Integration Tests)** - Level 2 - Will test the complete application including frontend

## Files to Create

### 1. `frontend/package.json`
Package configuration with all necessary dependencies:

```json
{
  "name": "parallel-task-execution-test-frontend",
  "version": "0.1.0",
  "private": true,
  "dependencies": {
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "react-router-dom": "^6.14.2",
    "axios": "^1.4.0",
    "@mui/material": "^5.14.0",
    "@mui/icons-material": "^5.14.0",
    "@emotion/react": "^11.11.1",
    "@emotion/styled": "^11.11.0"
  },
  "scripts": {
    "start": "react-scripts start",
    "build": "react-scripts build",
    "test": "react-scripts test",
    "eject": "react-scripts eject"
  },
  "eslintConfig": {
    "extends": [
      "react-app",
      "react-app/jest"
    ]
  },
  "browserslist": {
    "production": [
      ">0.2%",
      "not dead",
      "not op_mini all"
    ],
    "development": [
      "last 1 chrome version",
      "last 1 firefox version",
      "last 1 safari version"
    ]
  },
  "devDependencies": {
    "react-scripts": "5.0.1"
  }
}
```

**Dependencies:**
- `react` & `react-dom`: Core React framework
- `react-router-dom`: Client-side routing
- `axios`: HTTP client for API calls
- `@mui/material` & `@mui/icons-material`: Material-UI components
- `@emotion/react` & `@emotion/styled`: CSS-in-JS for Material-UI
- `react-scripts`: Build tooling and dev server

### 2. `frontend/src/App.js`
Main application component with routing and theming:

```jsx
import React from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import { ThemeProvider, createTheme } from '@mui/material/styles';
import CssBaseline from '@mui/material/CssBaseline';

// Import components
import Header from './components/Header';
import Footer from './components/Footer';
import HomePage from './components/HomePage';
import ProductList from './components/ProductList';
import ProductDetail from './components/ProductDetail';
import Cart from './components/Cart';
import Login from './components/Login';
import Register from './components/Register';

// Create theme
const theme = createTheme({
  palette: {
    primary: {
      main: '#1976d2',
    },
    secondary: {
      main: '#dc004e',
    },
  },
});

function App() {
  return (
    <ThemeProvider theme={theme}>
      <CssBaseline />
      <Router>
        <Header />
        <main style={{ padding: '20px', minHeight: 'calc(100vh - 130px)' }}>
          <Routes>
            <Route path="/" element={<HomePage />} />
            <Route path="/products" element={<ProductList />} />
            <Route path="/products/:id" element={<ProductDetail />} />
            <Route path="/cart" element={<Cart />} />
            <Route path="/login" element={<Login />} />
            <Route path="/register" element={<Register />} />
          </Routes>
        </main>
        <Footer />
      </Router>
    </ThemeProvider>
  );
}

export default App;
```

### 3. Component Files

**`frontend/src/components/Header.js`** - Application header with navigation:
- AppBar with logo and navigation links
- Shopping cart badge showing item count
- Login/Logout button (placeholder logic)

**`frontend/src/components/Footer.js`** - Application footer:
- Copyright information
- Simple footer layout

**`frontend/src/components/HomePage.js`** - Landing page:
- Welcome message
- Call-to-action button to product listing

**`frontend/src/components/ProductList.js`** - Product catalog view:
- Grid layout of product cards
- Placeholder product data
- Links to product detail pages
- Add to cart buttons (placeholder)

**`frontend/src/components/ProductDetail.js`** - Individual product view:
- Product details display
- Add to cart functionality (placeholder)
- Back to products link

**`frontend/src/components/Cart.js`** - Shopping cart view:
- List of cart items
- Quantity controls (placeholder)
- Remove item buttons (placeholder)
- Total price calculation (placeholder)

**`frontend/src/components/Login.js`** - Login form:
- Email and password fields
- Login button (placeholder submission)
- Link to register page

**`frontend/src/components/Register.js`** - Registration form:
- Username, email, password fields
- Register button (placeholder submission)
- Link to login page

## Implementation Steps

1. **Create Frontend Directory Structure**
   - Create `frontend/` directory in project root
   - Create `frontend/src/` subdirectory
   - Create `frontend/src/components/` subdirectory

2. **Set Up Package Configuration**
   - Create `frontend/package.json` with all dependencies
   - Specify correct versions for React 18 and Material-UI 5
   - Configure scripts for development and production

3. **Implement Main Application**
   - Create `frontend/src/App.js` with routing setup
   - Configure Material-UI theme with primary/secondary colors
   - Set up route definitions for all pages
   - Add ThemeProvider and CssBaseline for consistent styling

4. **Create Layout Components**
   - Implement Header component with AppBar and navigation
   - Implement Footer component with copyright
   - Ensure consistent layout across all pages

5. **Create Page Components**
   - Implement HomePage with welcome message
   - Implement ProductList with grid layout and placeholder data
   - Implement ProductDetail as placeholder
   - Implement Cart, Login, Register as placeholders
   - Use Material-UI components for consistent design

6. **Validation**
   - Verify all files are created
   - Check that package.json has correct dependencies
   - Ensure component imports and exports are correct
   - Run `npm install` to verify dependencies resolve
   - Run `npm start` to verify application loads

## Technical Considerations

### Framework Choice
- **React 18**: Latest stable version with modern features
- **Material-UI 5**: Comprehensive component library
- **React Router 6**: Modern routing with hooks

### Component Architecture
- Functional components with hooks (modern React pattern)
- Simple prop-based data flow
- No state management library needed for test project
- Placeholder logic for backend integration

### Styling Approach
- Material-UI components for consistent design
- Custom theme with primary/secondary colors
- Minimal custom styling
- Responsive design with Material-UI's Grid

### Development Experience
- Hot reload with react-scripts
- No build configuration needed (Create React App approach)
- Simple folder structure

## Integration Points

- **Task 7 (Integration Tests)**: Will verify frontend serves correctly
- **Future Backend Integration**: Components have placeholders for API calls with axios
- **Authentication Flow**: Login/Register components ready for backend integration
- **Shopping Cart**: Cart component ready to connect with Task 5's cart API

## Risks and Mitigation

**Risk**: npm dependencies might have version conflicts
- **Mitigation**: Using specific version ranges known to work together

**Risk**: Component imports might fail
- **Mitigation**: Following standard React component patterns

**Risk**: Material-UI setup might be incorrect
- **Mitigation**: Using standard Material-UI setup pattern with ThemeProvider

## Success Criteria

1. ✅ `frontend/package.json` exists with all required dependencies
2. ✅ Correct versions specified: React 18.2.0, Material-UI 5.14.0
3. ✅ `frontend/src/App.js` exists with routing configuration
4. ✅ Material-UI theme configured with primary/secondary colors
5. ✅ All 8 component files created in `frontend/src/components/`
6. ✅ Header component with AppBar and navigation
7. ✅ ProductList component with grid layout and placeholder data
8. ✅ All routes defined: /, /products, /products/:id, /cart, /login, /register
9. ✅ Components use Material-UI components consistently
10. ✅ Application can be started with `npm install && npm start`
11. ✅ No console errors on initial load
12. ✅ Navigation between pages works correctly

## Estimated Effort
**35 minutes** - Frontend structure creation, component implementation with Material-UI, and routing setup

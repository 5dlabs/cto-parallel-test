# Task 6: Frontend Components - Agent Prompt

You are a React frontend developer tasked with creating the UI structure for an e-commerce test application.

## Your Mission
Set up a React application with routing, Material-UI components, and placeholder pages. Create a complete frontend structure that can later be connected to the backend APIs.

## What You Must Create

### 1. Create `frontend/package.json`
Include dependencies:
- react 18.2.0, react-dom 18.2.0
- react-router-dom 6.14.2
- axios 1.4.0
- @mui/material 5.14.0, @mui/icons-material 5.14.0
- @emotion/react 11.11.1, @emotion/styled 11.11.0
- react-scripts 5.0.1 (devDependencies)

Scripts: start, build, test, eject

### 2. Create `frontend/src/App.js`
- Import Router, Routes, Route from react-router-dom
- Import ThemeProvider, createTheme from @mui/material
- Import CssBaseline
- Create theme with primary (#1976d2) and secondary (#dc004e) colors
- Set up Router with Header, Routes, and Footer
- Define routes for: /, /products, /products/:id, /cart, /login, /register

### 3. Create Layout Components

**frontend/src/components/Header.js**:
- Material-UI AppBar with Toolbar
- App title linking to /
- Products link
- Cart icon with badge (placeholder count)
- Login/Logout button (placeholder state)

**frontend/src/components/Footer.js**:
- Centered content with current year
- Material-UI Box and Container

### 4. Create Page Components

**frontend/src/components/HomePage.js**:
- Welcome message
- "Shop Now" button linking to /products

**frontend/src/components/ProductList.js**:
- Grid of product cards (3 hardcoded products)
- Each card: image placeholder, name, price, description
- "View Details" and "Add to Cart" buttons

**frontend/src/components/ProductDetail.js**:
- Extract id from route params
- Display "Product Detail - ID: {id}"
- Placeholder for future product details

**frontend/src/components/Cart.js**:
- Display "Shopping Cart" heading
- Placeholder for cart items

**frontend/src/components/Login.js**:
- Login form with email/password fields
- Submit button (no functionality yet)

**frontend/src/components/Register.js**:
- Registration form fields
- Submit button (no functionality yet)

## Key Requirements

✅ **All components use Material-UI**
✅ **Navigation works between all routes**
✅ **Responsive layout**
✅ **Consistent theme applied**
✅ **No backend integration (placeholder data)**

## Success Definition
- `npm install` succeeds
- `npm start` launches app on port 3000
- All routes accessible
- Navigation works
- Components render correctly

## Context
This is a **Level 0** task - no dependencies. Frontend and backend develop in parallel.

---

**Start working now. Create the React app structure and verify all components render.**

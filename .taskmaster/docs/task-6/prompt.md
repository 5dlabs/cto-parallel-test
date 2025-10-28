# Autonomous Agent Prompt: Frontend Components

## Mission
You are tasked with creating a basic React frontend structure with Material-UI components and React Router navigation. This provides the user interface foundation for an e-commerce application test project.

## Prerequisites
**None** - This is a Level 0 task with no dependencies. You can proceed immediately.

## What You Need to Do

### 1. Create Package Configuration (`frontend/package.json`)

Create the file with these exact dependencies:

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

### 2. Create Main App Component (`frontend/src/App.js`)

Create the main component with routing - see full code in task.txt file, section 2.

**Key elements:**
- Import React Router components (BrowserRouter, Routes, Route)
- Import Material-UI theming (ThemeProvider, createTheme, CssBaseline)
- Import all 8 component files
- Create theme with primary color #1976d2, secondary #dc004e
- Define 6 routes: /, /products, /products/:id, /cart, /login, /register
- Wrap in ThemeProvider > Router > Header + Routes + Footer

### 3. Create Component Files

Create these 8 files in `frontend/src/components/`:

**Header.js** - Use full code from task.txt section 3
- AppBar with site logo/title
- Navigation buttons (Products, Cart, Login)
- Shopping cart badge icon with count
- Material-UI components: AppBar, Toolbar, Typography, Button, IconButton, Badge

**Footer.js** - Use full code from task.txt section 3
- Simple footer with copyright
- Material-UI components: Box, Typography, Container

**HomePage.js** - Use full code from task.txt section 3
- Welcome heading and subheading
- "Shop Now" button linking to /products
- Material-UI components: Container, Typography, Button, Box

**ProductList.js** - Use full code from task.txt section 3
- Placeholder array of 3 products
- Grid layout (3 columns on desktop)
- Card for each product with image placeholder, name, price, description
- "View Details" and "Add to Cart" buttons
- Material-UI components: Container, Typography, Grid, Card, CardContent, CardMedia, CardActions, Button

**ProductDetail.js** - Create placeholder component
- Use useParams() to get product ID from route
- Display "Product Detail" heading and product ID
- "Back to Products" link
- Material-UI components similar to ProductList

**Cart.js** - Create placeholder component
- Display "Shopping Cart" heading
- Empty cart message or placeholder items
- Material-UI components: Container, Typography

**Login.js** - Create placeholder component
- Email and password TextField components
- Login Button
- Link to Register page
- Material-UI components: Container, TextField, Button, Typography, Box

**Register.js** - Create placeholder component
- Username, email, password TextField components
- Register Button
- Link to Login page
- Material-UI components: Container, TextField, Button, Typography, Box

**Note**: Use the full component code from the task.txt file. Components should be functional components using hooks, not class components.

## Expected Behavior

After implementation:
- Running `npm install` in frontend/ directory installs all dependencies
- Running `npm start` launches development server on port 3000
- Application loads without console errors
- Navigation between all pages works
- Material-UI styling is consistent across components
- Responsive layout works on different screen sizes

## Validation Steps

1. **Directory Structure**
   ```bash
   ls -la frontend/package.json
   ls -la frontend/src/App.js
   ls -la frontend/src/components/
   ```

2. **Install Dependencies**
   ```bash
   cd frontend && npm install
   ```
   Should complete without errors.

3. **Start Development Server**
   ```bash
   cd frontend && npm start
   ```
   Should start on http://localhost:3000

4. **Check Browser**
   - Open http://localhost:3000
   - Should see home page with "Welcome to Our Store"
   - Click navigation links to verify routing
   - Check browser console for errors (should be none)

## Constraints

- This is a test project - components have placeholder functionality
- Use functional components with hooks, not class components
- Follow exact dependency versions specified
- Use Material-UI components for all UI elements
- Do not add backend API integration (placeholders only)
- Keep components simple and focused on structure

## Common Issues and Solutions

**Issue**: npm install fails with dependency conflicts
- **Solution**: Ensure exact versions in package.json match specification

**Issue**: Material-UI theme not applying
- **Solution**: Verify ThemeProvider wraps entire Router in App.js

**Issue**: Routing not working
- **Solution**: Ensure BrowserRouter wraps all Routes, and Route elements use correct syntax

**Issue**: Components not rendering
- **Solution**: Check import paths are correct, components are exported with 'export default'

## Success Definition

Task is complete when:
- ✅ package.json created with all dependencies
- ✅ App.js created with routing and theming
- ✅ All 8 component files created
- ✅ npm install completes successfully
- ✅ npm start launches without errors
- ✅ Application loads in browser
- ✅ All navigation links work
- ✅ Material-UI styling is visible
- ✅ No console errors on initial load

## Integration Notes

- **Task 7 (Integration Tests)** will verify the frontend serves correctly
- Components have placeholder data and functions for now
- Future tasks could integrate with backend APIs (Tasks 3, 4, 5)
- Authentication components ready for JWT integration
- Cart component ready for shopping cart API integration

## Next Steps

After completing this task:
1. Task 7 (Integration Tests) can begin testing the complete application
2. Components can be enhanced with real API integration
3. State management can be added if needed
4. Full e-commerce functionality can be implemented

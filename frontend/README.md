# E-Commerce Frontend

React-based frontend application for the e-commerce test platform.

## Technology Stack

- **React**: 18.2.0
- **React Router**: 6.14.2 (client-side routing)
- **Material-UI**: 5.14.0 (UI components)
- **Axios**: 1.4.0 (HTTP client for future API integration)
- **Emotion**: CSS-in-JS styling (required by Material-UI)

## Getting Started

### Installation

```bash
npm install
```

### Development

Start the development server on port 3000:

```bash
npm start
```

The application will open in your browser at [http://localhost:3000](http://localhost:3000).

### Build

Create a production build:

```bash
npm run build
```

The optimized production build will be created in the `build/` directory.

### Testing

Run tests:

```bash
npm test
```

## Project Structure

```
frontend/
├── public/
│   └── index.html           # HTML template
├── src/
│   ├── components/          # React components
│   │   ├── Header.js       # Navigation header with cart badge
│   │   ├── Footer.js       # Page footer
│   │   ├── HomePage.js     # Landing page with hero section
│   │   ├── ProductList.js  # Product catalog with grid layout
│   │   ├── ProductDetail.js # Product detail page (placeholder)
│   │   ├── Cart.js         # Shopping cart (placeholder)
│   │   ├── Login.js        # Login form (placeholder)
│   │   └── Register.js     # Registration form (placeholder)
│   ├── App.js              # Main app component with routing
│   └── index.js            # Application entry point
└── package.json            # Dependencies and scripts
```

## Routes

- `/` - Home page
- `/products` - Product catalog
- `/products/:id` - Product detail page
- `/cart` - Shopping cart
- `/login` - User login
- `/register` - User registration

## Features

### Current Implementation

✅ React application with routing  
✅ Material-UI themed components  
✅ Responsive layout  
✅ Navigation between all pages  
✅ Product catalog with hardcoded data  
✅ Form components for login/registration  
✅ Shopping cart placeholder  

### Placeholder Data

Currently, the application uses placeholder/hardcoded data:
- 3 sample products in the product list
- Mock authentication state (not logged in)
- Empty shopping cart

### Future Integration

The following features will be implemented in future tasks:

🔄 Connect to backend REST APIs  
🔄 User authentication with JWT tokens  
🔄 Real product data from API  
🔄 Functional shopping cart  
🔄 Order placement and checkout  
🔄 State management (Context API or Redux)  
🔄 Error handling and loading states  
🔄 Unit and integration tests  

## Material-UI Theme

The application uses a custom Material-UI theme:

- **Primary Color**: `#1976d2` (Blue)
- **Secondary Color**: `#dc004e` (Pink)

## Development Notes

### Component Design

All components are functional components using React Hooks. The application follows React best practices:

- Components are self-contained and reusable
- Props are used for data flow
- No complex state management (kept simple for initial implementation)

### Styling

Styling is handled via Material-UI's `sx` prop and theme system, providing:
- Consistent design language
- Responsive components out of the box
- Easy theme customization
- Built-in accessibility features

### Code Quality

The application is built with Create React App and includes:
- ESLint configuration (react-app)
- Modern JavaScript (ES6+)
- No build warnings or errors

## Browser Support

The application targets modern browsers as specified in `package.json`:

**Production:**
- >0.2% market share
- Not dead browsers
- Not Opera Mini

**Development:**
- Latest Chrome
- Latest Firefox
- Latest Safari

## License

This project is part of the CTO parallel test implementation.

# E-Commerce Frontend Application

A React-based frontend for an e-commerce test application built with Material-UI.

## Features

- **Routing**: React Router DOM for navigation between pages
- **Material-UI**: Consistent design system with theme customization
- **Responsive Layout**: Mobile-friendly components
- **Placeholder Components**: Ready for backend API integration

## Routes

- `/` - Home page with welcome message
- `/products` - Product catalog with grid layout
- `/products/:id` - Product detail page (placeholder)
- `/cart` - Shopping cart (placeholder)
- `/login` - Login form
- `/register` - Registration form

## Getting Started

### Prerequisites

- Node.js 14 or higher
- npm or yarn

### Installation

```bash
npm install
```

### Development

Start the development server:

```bash
npm start
```

The application will open at [http://localhost:3000](http://localhost:3000).

### Build

Create a production build:

```bash
npm run build
```

### Test

Run tests:

```bash
npm test
```

## Project Structure

```
frontend/
├── public/
│   └── index.html          # HTML template
├── src/
│   ├── App.js              # Main application component with routing
│   ├── index.js            # Entry point
│   └── components/
│       ├── Header.js       # Navigation header
│       ├── Footer.js       # Page footer
│       ├── HomePage.js     # Landing page
│       ├── ProductList.js  # Product catalog
│       ├── ProductDetail.js # Product details
│       ├── Cart.js         # Shopping cart
│       ├── Login.js        # Login form
│       └── Register.js     # Registration form
└── package.json
```

## Technology Stack

- **React** 18.2.0 - UI framework
- **React Router DOM** 6.14.2 - Client-side routing
- **Material-UI** 5.14.0 - Component library
- **Axios** 1.4.0 - HTTP client (for future API calls)
- **Emotion** - CSS-in-JS styling

## Theme

The application uses a custom Material-UI theme with:
- Primary color: `#1976d2` (blue)
- Secondary color: `#dc004e` (pink)

## Future Enhancements

- Connect to backend REST APIs
- Implement state management (Context API or Redux)
- Add authentication flow with JWT tokens
- Implement cart functionality
- Add form validation
- Error handling and loading states
- Unit and integration tests

## Notes

This is a placeholder implementation with hardcoded data. All components are ready for API integration when backend services are available.

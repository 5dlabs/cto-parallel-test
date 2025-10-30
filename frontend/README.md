# E-Commerce Frontend

React-based frontend for the e-commerce test application.

## Features

- React 18.2.0 with React Router for navigation
- Material-UI 5.14.0 for component library
- Responsive design
- Placeholder pages ready for backend integration

## Prerequisites

- Node.js 14+ and npm

## Installation

```bash
npm install
```

## Running the Application

### Development Mode
```bash
npm start
```

The application will start on [http://localhost:3000](http://localhost:3000).

### Production Build
```bash
npm run build
```

The optimized build will be created in the `build/` directory.

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
│   ├── App.js              # Main application with routing
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

## Routes

- `/` - Home page
- `/products` - Product listing
- `/products/:id` - Product detail
- `/cart` - Shopping cart
- `/login` - User login
- `/register` - User registration

## Technology Stack

- **React**: UI framework
- **React Router DOM**: Client-side routing
- **Material-UI**: Component library
- **Axios**: HTTP client (for future API integration)
- **Emotion**: CSS-in-JS styling

## Future Integration

This frontend is designed to integrate with the Rust backend API. Future enhancements will include:

- API integration using Axios
- Authentication state management
- Shopping cart state management
- Real product data from backend
- Error handling and loading states

## Notes

- Currently uses placeholder data (no backend integration)
- Authentication state is mocked
- Cart functionality is placeholder only
- All components use Material-UI for consistent styling

# E-Commerce Frontend

A React-based e-commerce frontend built with shadcn/ui, Tailwind CSS, and React Router.

## Features

- **React 18.2.0**: Latest stable React version
- **React Router DOM 6.14.2**: Client-side routing
- **shadcn/ui**: High-quality, accessible component library
- **Tailwind CSS**: Utility-first CSS framework
- **Vite**: Fast build tool and development server
- **Responsive Design**: Mobile-first approach

## Components

### Layout Components
- **Header**: Navigation bar with cart badge and mobile menu
- **Footer**: Site footer with links

### Page Components
- **HomePage**: Landing page with hero section and features
- **ProductList**: Grid view of all products
- **ProductDetail**: Individual product page with add to cart
- **Cart**: Shopping cart with quantity controls
- **Login**: User login form
- **Register**: User registration form

### UI Components (shadcn/ui)
- Button
- Card
- Badge
- Input
- Label

## Getting Started

### Installation

```bash
npm install --production=false
```

Note: Use `--production=false` to ensure devDependencies (like Vite) are installed.

### Development

Start the development server:

```bash
npm start
# or
npm run dev
```

The app will be available at http://localhost:3000

### Building for Production

```bash
npm run build
```

The production build will be created in the `dist/` directory.

### Preview Production Build

```bash
npm run preview
```

## Routes

- `/` - Home page
- `/products` - Product listing
- `/products/:id` - Product detail
- `/cart` - Shopping cart
- `/login` - User login
- `/register` - User registration

## Project Structure

```
frontend/
├── src/
│   ├── components/
│   │   ├── ui/          # shadcn/ui components
│   │   ├── Header.jsx
│   │   ├── Footer.jsx
│   │   ├── HomePage.jsx
│   │   ├── ProductList.jsx
│   │   ├── ProductDetail.jsx
│   │   ├── Cart.jsx
│   │   ├── Login.jsx
│   │   └── Register.jsx
│   ├── lib/
│   │   └── utils.js     # Utility functions
│   ├── App.jsx          # Main app component
│   ├── main.jsx         # Entry point
│   └── index.css        # Global styles
├── public/
├── index.html
├── vite.config.js
├── tailwind.config.js
├── postcss.config.js
├── components.json      # shadcn/ui config
└── package.json
```

## Configuration

### shadcn/ui Configuration

The project uses the following shadcn/ui settings:
- Style: Default
- Base color: Slate
- CSS variables: Yes
- JavaScript (not TypeScript)

### Tailwind CSS

Tailwind CSS is configured with:
- Custom color variables for theming
- Responsive breakpoints
- shadcn/ui compatible plugins

## Future Enhancements

- Connect to backend API for real data
- Add state management (Context API or Redux)
- Implement authentication with JWT
- Add product search and filtering
- Implement real cart persistence
- Add payment integration
- Add order history
- Implement user profile management

## Testing

```bash
npm test
```

Currently configured to pass without actual tests. Tests will be added in future iterations.

## License

This project is part of the e-commerce application test suite.

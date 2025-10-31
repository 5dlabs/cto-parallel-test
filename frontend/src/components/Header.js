import React from 'react';
import { Link as RouterLink } from 'react-router-dom';
import {
  AppBar,
  Toolbar,
  Typography,
  Button,
  IconButton,
  Badge,
  Box,
  Link,
} from '@mui/material';
import ShoppingCartIcon from '@mui/icons-material/ShoppingCart';

const Header = () => {
  const isAuthenticated = false;
  const cartItemCount = 0;

  return (
    <AppBar position="static" color="primary" enableColorOnDark>
      <Toolbar sx={{ display: 'flex', alignItems: 'center', gap: 2 }}>
        <Typography
          variant="h6"
          component={RouterLink}
          to="/"
          sx={{ color: 'inherit', textDecoration: 'none', fontWeight: 600 }}
        >
          E-Shop
        </Typography>
        <Box sx={{ flexGrow: 1 }}>
          <Link
            component={RouterLink}
            to="/products"
            color="inherit"
            underline="none"
            sx={{ fontWeight: 500, ml: 3 }}
          >
            Products
          </Link>
        </Box>
        <IconButton
          size="large"
          edge="end"
          color="inherit"
          component={RouterLink}
          to="/cart"
          aria-label="shopping cart"
        >
          <Badge badgeContent={cartItemCount} color="secondary" overlap="circular">
            <ShoppingCartIcon />
          </Badge>
        </IconButton>
        <Button
          color="inherit"
          component={RouterLink}
          to={isAuthenticated ? '/logout' : '/login'}
        >
          {isAuthenticated ? 'Logout' : 'Login'}
        </Button>
      </Toolbar>
    </AppBar>
  );
};

export default Header;

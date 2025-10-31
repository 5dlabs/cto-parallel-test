import React from 'react';
import {
  AppBar,
  Toolbar,
  Typography,
  Button,
  IconButton,
  Badge,
  Box,
} from '@mui/material';
import ShoppingCartIcon from '@mui/icons-material/ShoppingCart';
import { Link as RouterLink } from 'react-router-dom';

const Header = () => {
  const cartCount = 0;
  const isLoggedIn = false;

  return (
    <AppBar position="static" color="primary">
      <Toolbar sx={{ gap: 2 }}>
        <Typography
          variant="h6"
          component={RouterLink}
          to="/"
          sx={{
            color: 'inherit',
            textDecoration: 'none',
            flexGrow: 1,
            fontWeight: 600,
          }}
        >
          ShopSmart
        </Typography>
        <Box sx={{ display: 'flex', alignItems: 'center', gap: { xs: 1, sm: 2 } }}>
          <Button color="inherit" component={RouterLink} to="/products">
            Products
          </Button>
          <IconButton
            color="inherit"
            component={RouterLink}
            to="/cart"
            aria-label="View cart"
          >
            <Badge badgeContent={cartCount} color="secondary" overlap="circular">
              <ShoppingCartIcon />
            </Badge>
          </IconButton>
          {isLoggedIn ? (
            <Button color="inherit">Logout</Button>
          ) : (
            <Button color="inherit" component={RouterLink} to="/login">
              Login
            </Button>
          )}
        </Box>
      </Toolbar>
    </AppBar>
  );
};

export default Header;

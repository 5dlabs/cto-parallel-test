import React from 'react';
import { Link as RouterLink, useNavigate } from 'react-router-dom';
import AppBar from '@mui/material/AppBar';
import Toolbar from '@mui/material/Toolbar';
import Typography from '@mui/material/Typography';
import Button from '@mui/material/Button';
import IconButton from '@mui/material/IconButton';
import Badge from '@mui/material/Badge';
import Box from '@mui/material/Box';
import Link from '@mui/material/Link';
import ShoppingCartIcon from '@mui/icons-material/ShoppingCart';

function Header() {
  const [isAuthenticated, setIsAuthenticated] = React.useState(false);
  const navigate = useNavigate();

  const handleAuthClick = () => {
    setIsAuthenticated((prev) => !prev);
    if (!isAuthenticated) {
      navigate('/login');
    }
  };

  return (
    <AppBar position="static" color="primary">
      <Toolbar sx={{ display: 'flex', gap: 2 }}>
        <Typography
          variant="h6"
          component={RouterLink}
          to="/"
          sx={{ flexGrow: 1, color: 'inherit', textDecoration: 'none' }}
        >
          ShopSmart
        </Typography>
        <Box sx={{ display: 'flex', alignItems: 'center', gap: 2 }}>
          <Link
            component={RouterLink}
            to="/products"
            underline="none"
            color="inherit"
            variant="button"
            sx={{ fontWeight: 500 }}
          >
            Products
          </Link>
          <IconButton component={RouterLink} to="/cart" color="inherit" aria-label="Cart">
            <Badge badgeContent={0} color="secondary">
              <ShoppingCartIcon />
            </Badge>
          </IconButton>
          <Button color="inherit" onClick={handleAuthClick}>
            {isAuthenticated ? 'Logout' : 'Login'}
          </Button>
        </Box>
      </Toolbar>
    </AppBar>
  );
}

export default Header;

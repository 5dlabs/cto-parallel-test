import AppBar from '@mui/material/AppBar';
import Toolbar from '@mui/material/Toolbar';
import Typography from '@mui/material/Typography';
import Button from '@mui/material/Button';
import IconButton from '@mui/material/IconButton';
import Badge from '@mui/material/Badge';
import Box from '@mui/material/Box';
import Stack from '@mui/material/Stack';
import ShoppingCartIcon from '@mui/icons-material/ShoppingCart';
import { Link as RouterLink, useLocation } from 'react-router-dom';

const Header = () => {
  const isLoggedIn = false;
  const cartItemCount = 0;
  const location = useLocation();

  return (
    <AppBar position="static" color="primary">
      <Toolbar>
        <Typography
          variant="h6"
          component={RouterLink}
          to="/"
          sx={{ flexGrow: 1, textDecoration: 'none', color: 'inherit', fontWeight: 600 }}
        >
          ShopSmart
        </Typography>
        <Stack direction="row" spacing={2} alignItems="center">
          <Button
            color="inherit"
            component={RouterLink}
            to="/products"
            variant={location.pathname.startsWith('/products') ? 'outlined' : 'text'}
          >
            Products
          </Button>
          <IconButton
            size="large"
            aria-label="show cart items"
            color="inherit"
            component={RouterLink}
            to="/cart"
          >
            <Badge badgeContent={cartItemCount} color="secondary">
              <ShoppingCartIcon />
            </Badge>
          </IconButton>
          <Box>
            {isLoggedIn ? (
              <Button color="inherit">Logout</Button>
            ) : (
              <Button color="inherit" component={RouterLink} to="/login">
                Login
              </Button>
            )}
          </Box>
        </Stack>
      </Toolbar>
    </AppBar>
  );
};

export default Header;

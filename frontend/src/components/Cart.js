import React from 'react';
import { Link as RouterLink } from 'react-router-dom';
import {
  Container,
  Typography,
  Button,
  Box,
  Paper,
  Divider,
} from '@mui/material';
import ShoppingCartIcon from '@mui/icons-material/ShoppingCart';

function Cart() {
  // TODO: Connect to cart state/API
  // This is a placeholder component

  return (
    <Container maxWidth="md">
      <Box sx={{ my: 4 }}>
        <Typography variant="h3" component="h1" gutterBottom>
          <ShoppingCartIcon sx={{ mr: 1, verticalAlign: 'middle' }} />
          Shopping Cart
        </Typography>

        <Paper elevation={3} sx={{ p: 4, mt: 3 }}>
          <Typography variant="body1" color="text.secondary" paragraph>
            This is a placeholder for the shopping cart. In a full implementation,
            this page would display:
          </Typography>

          <Box component="ul" sx={{ pl: 3 }}>
            <li>
              <Typography variant="body2">List of items in the cart</Typography>
            </li>
            <li>
              <Typography variant="body2">Quantity controls for each item</Typography>
            </li>
            <li>
              <Typography variant="body2">Remove item buttons</Typography>
            </li>
            <li>
              <Typography variant="body2">Subtotal and total calculations</Typography>
            </li>
            <li>
              <Typography variant="body2">Checkout button</Typography>
            </li>
          </Box>

          <Divider sx={{ my: 3 }} />

          <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
            <Typography variant="h5">
              Total: $0.00
            </Typography>
            <Button variant="contained" size="large" disabled>
              Proceed to Checkout
            </Button>
          </Box>

          <Box sx={{ mt: 3 }}>
            <Button
              component={RouterLink}
              to="/products"
              variant="outlined"
            >
              Continue Shopping
            </Button>
          </Box>

          <Typography variant="body2" color="text.secondary" sx={{ mt: 3 }}>
            TODO: Fetch cart items from the API and implement cart functionality
          </Typography>
        </Paper>
      </Box>
    </Container>
  );
}

export default Cart;

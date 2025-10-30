import React from 'react';
import { Link as RouterLink } from 'react-router-dom';
import Container from '@mui/material/Container';
import Typography from '@mui/material/Typography';
import Button from '@mui/material/Button';
import Box from '@mui/material/Box';
import Paper from '@mui/material/Paper';

function Cart() {
  // TODO: Connect to cart API

  return (
    <Container maxWidth="md">
      <Typography variant="h4" component="h1" gutterBottom>
        Shopping Cart
      </Typography>

      <Paper elevation={3} sx={{ p: 4, mt: 3 }}>
        <Typography variant="body1" color="text.secondary" paragraph>
          Your cart is empty. This is a placeholder for cart items. In the future,
          this component will display items from the cart API.
        </Typography>

        <Box sx={{ mt: 3 }}>
          <Typography variant="h6" gutterBottom>
            Cart Total: $0.00
          </Typography>

          <Box sx={{ mt: 3 }}>
            <Button
              variant="outlined"
              component={RouterLink}
              to="/products"
              sx={{ mr: 2 }}
            >
              Continue Shopping
            </Button>
            <Button variant="contained" color="primary" disabled>
              Checkout
            </Button>
          </Box>
        </Box>
      </Paper>
    </Container>
  );
}

export default Cart;

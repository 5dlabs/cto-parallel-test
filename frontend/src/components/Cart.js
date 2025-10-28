import React from 'react';
import { Link as RouterLink } from 'react-router-dom';
import { Container, Typography, Button, Box, Paper } from '@mui/material';

function Cart() {
  // TODO: Connect to cart state and display actual cart items

  return (
    <Container maxWidth="md">
      <Paper sx={{ p: 4, mt: 4 }}>
        <Typography variant="h3" component="h1" gutterBottom>
          Shopping Cart
        </Typography>
        <Typography variant="body1" color="text.secondary" paragraph>
          Your cart is empty. This is a placeholder for cart items.
          In the future, this will display items from your shopping cart.
        </Typography>
        <Box sx={{ mt: 4, display: 'flex', justifyContent: 'space-between' }}>
          <Button
            variant="outlined"
            component={RouterLink}
            to="/products"
          >
            Continue Shopping
          </Button>
          <Button variant="contained" disabled>
            Checkout
          </Button>
        </Box>
      </Paper>
    </Container>
  );
}

export default Cart;

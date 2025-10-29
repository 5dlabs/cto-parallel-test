import React from 'react';
import { Container, Typography, Paper } from '@mui/material';

function Cart() {
  return (
    <Container maxWidth="md">
      <Typography variant="h3" component="h1" gutterBottom>
        Shopping Cart
      </Typography>

      <Paper elevation={2} sx={{ p: 3, mt: 3 }}>
        <Typography variant="body1" color="text.secondary">
          Your cart is currently empty. This is a placeholder for the shopping cart
          functionality. In a future implementation, this page will display cart items,
          quantities, prices, and checkout options.
        </Typography>
      </Paper>
    </Container>
  );
}

export default Cart;

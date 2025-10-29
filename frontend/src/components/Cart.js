import React from 'react';
import Container from '@mui/material/Container';
import Typography from '@mui/material/Typography';
import Box from '@mui/material/Box';

function Cart() {
  return (
    <Container maxWidth="md">
      <Box sx={{ mt: 4 }}>
        <Typography variant="h3" component="h1" gutterBottom>
          Shopping Cart
        </Typography>
        <Typography variant="body1" color="text.secondary" paragraph>
          Your cart is currently empty. This is a placeholder that will display cart items
          once connected to the backend API.
        </Typography>
      </Box>
    </Container>
  );
}

export default Cart;

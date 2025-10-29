import React from 'react';
import { Container, Typography, Box } from '@mui/material';

function Cart() {
  return (
    <Container maxWidth="md">
      <Box sx={{ mt: 4 }}>
        <Typography variant="h3" component="h1" gutterBottom>
          Shopping Cart
        </Typography>
        
        <Typography variant="body1" color="text.secondary" paragraph>
          Your cart items will be displayed here.
        </Typography>
        
        <Typography variant="body2" color="text.secondary">
          This is a placeholder component. Future implementation will display cart items from the API.
        </Typography>
      </Box>
    </Container>
  );
}

export default Cart;

import React from 'react';
import Container from '@mui/material/Container';
import Typography from '@mui/material/Typography';
import Paper from '@mui/material/Paper';
import Box from '@mui/material/Box';

function Cart() {
  // TODO: Connect to cart API
  return (
    <Container maxWidth="md">
      <Typography variant="h3" component="h1" gutterBottom sx={{ mt: 2 }}>
        Shopping Cart
      </Typography>
      <Paper elevation={3} sx={{ p: 4, mt: 3 }}>
        <Typography variant="body1" color="text.secondary">
          This is a placeholder for your shopping cart.
          Future implementation will display:
        </Typography>
        <Box component="ul" sx={{ mt: 2 }}>
          <Typography component="li">Cart items with images and descriptions</Typography>
          <Typography component="li">Quantity controls for each item</Typography>
          <Typography component="li">Individual and total prices</Typography>
          <Typography component="li">Remove item functionality</Typography>
          <Typography component="li">Checkout button</Typography>
        </Box>
        <Box sx={{ mt: 3, p: 2, bgcolor: 'grey.100', borderRadius: 1 }}>
          <Typography variant="h6">
            Cart Total: $0.00
          </Typography>
        </Box>
      </Paper>
    </Container>
  );
}

export default Cart;

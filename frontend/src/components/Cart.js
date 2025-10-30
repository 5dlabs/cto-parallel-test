import React from 'react';
import Container from '@mui/material/Container';
import Typography from '@mui/material/Typography';
import Box from '@mui/material/Box';
import Paper from '@mui/material/Paper';

function Cart() {
  return (
    <Container maxWidth="md">
      <Typography variant="h4" component="h1" gutterBottom>
        Shopping Cart
      </Typography>
      <Paper elevation={2} sx={{ p: 4, mt: 3 }}>
        <Typography variant="body1" color="text.secondary">
          This is a placeholder for the shopping cart page. In the future,
          this component will display items added to the cart, allow quantity
          adjustments, and show the cart total.
        </Typography>
        <Box sx={{ mt: 3 }}>
          <Typography variant="h6">
            Cart Total: $0.00
          </Typography>
        </Box>
      </Paper>
    </Container>
  );
}

export default Cart;

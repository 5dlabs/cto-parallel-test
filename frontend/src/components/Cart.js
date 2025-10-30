import React from 'react';
import Container from '@mui/material/Container';
import Typography from '@mui/material/Typography';
import Paper from '@mui/material/Paper';
import Box from '@mui/material/Box';

function Cart() {
  // TODO: Fetch cart items from API

  return (
    <Container maxWidth="md">
      <Typography variant="h3" component="h1" gutterBottom sx={{ mb: 4 }}>
        Shopping Cart
      </Typography>
      <Paper elevation={3} sx={{ p: 4 }}>
        <Typography variant="body1" color="text.secondary">
          This is a placeholder for the shopping cart. In the future, this will display:
        </Typography>
        <Box component="ul" sx={{ mt: 2, ml: 2 }}>
          <Typography component="li" variant="body1">
            List of items in the cart
          </Typography>
          <Typography component="li" variant="body1">
            Quantity controls for each item
          </Typography>
          <Typography component="li" variant="body1">
            Total price calculation
          </Typography>
          <Typography component="li" variant="body1">
            Checkout button
          </Typography>
        </Box>
      </Paper>
    </Container>
  );
}

export default Cart;

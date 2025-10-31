import React from 'react';
import { Paper, Typography, Stack, Button } from '@mui/material';

const Cart = () => (
  <Paper sx={{ p: { xs: 3, md: 4 } }} elevation={2}>
    <Stack spacing={2}>
      <Typography variant="h4" component="h1">
        Shopping Cart
      </Typography>
      <Typography variant="body1" color="text.secondary">
        Your selected items will appear here. Add products to your cart to review them before
        checkout.
      </Typography>
      <Button variant="contained" color="primary" disabled sx={{ alignSelf: { xs: 'stretch', sm: 'flex-start' } }}>
        Proceed to Checkout
      </Button>
    </Stack>
  </Paper>
);

export default Cart;

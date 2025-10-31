import React from 'react';
import { Container, Typography, Paper, Stack } from '@mui/material';

const Cart = () => (
  <Container maxWidth="md">
    <Stack spacing={3}>
      <Typography variant="h4" component="h1" sx={{ fontWeight: 600 }}>
        Shopping Cart
      </Typography>
      <Paper elevation={2} sx={{ p: 3 }}>
        <Typography variant="body1" color="text.secondary">
          Your cart is currently empty. Add items to see them listed here.
        </Typography>
      </Paper>
    </Stack>
  </Container>
);

export default Cart;

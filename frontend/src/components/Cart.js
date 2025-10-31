import React from 'react';
import Typography from '@mui/material/Typography';
import Box from '@mui/material/Box';
import Paper from '@mui/material/Paper';

function Cart() {
  return (
    <Box sx={{ display: 'flex', flexDirection: 'column', gap: 3 }}>
      <Typography variant="h4" component="h2" fontWeight={600}>
        Shopping Cart
      </Typography>
      <Paper sx={{ p: 3 }} elevation={1}>
        <Typography variant="body1" color="text.secondary">
          Your cart is currently empty. Start adding products to see them here.
        </Typography>
      </Paper>
    </Box>
  );
}

export default Cart;

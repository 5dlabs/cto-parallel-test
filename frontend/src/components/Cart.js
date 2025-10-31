import React from 'react';
import { Link } from 'react-router-dom';
import Container from '@mui/material/Container';
import Typography from '@mui/material/Typography';
import Button from '@mui/material/Button';
import Box from '@mui/material/Box';
import Paper from '@mui/material/Paper';
import Divider from '@mui/material/Divider';

function Cart() {
  return (
    <Container maxWidth="md">
      <Typography variant="h3" component="h1" gutterBottom sx={{ mb: 4 }}>
        Shopping Cart
      </Typography>
      <Paper elevation={3} sx={{ p: 4 }}>
        <Typography variant="body1" color="text.secondary" paragraph>
          This is a placeholder for the shopping cart. In a full implementation, this would display:
        </Typography>
        <Box component="ul" sx={{ mb: 3 }}>
          <li>List of items in the cart</li>
          <li>Quantity controls for each item</li>
          <li>Item prices and subtotals</li>
          <li>Total price calculation</li>
          <li>Remove item buttons</li>
          <li>Checkout button</li>
        </Box>
        <Divider sx={{ my: 3 }} />
        <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
          <Typography variant="h5">
            Total: $0.00
          </Typography>
          <Box sx={{ display: 'flex', gap: 2 }}>
            <Button variant="outlined" component={Link} to="/products">
              Continue Shopping
            </Button>
            <Button variant="contained" color="primary">
              Checkout
            </Button>
          </Box>
        </Box>
      </Paper>
    </Container>
  );
}

export default Cart;

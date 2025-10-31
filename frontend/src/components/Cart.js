import React from 'react';
import { Link as RouterLink } from 'react-router-dom';
import Container from '@mui/material/Container';
import Typography from '@mui/material/Typography';
import Box from '@mui/material/Box';
import Button from '@mui/material/Button';
import Paper from '@mui/material/Paper';
import Divider from '@mui/material/Divider';

function Cart() {
  // TODO: Fetch cart items from API

  return (
    <Container maxWidth="md">
      <Box sx={{ my: 4 }}>
        <Typography variant="h3" component="h1" gutterBottom>
          Shopping Cart
        </Typography>

        <Paper elevation={3} sx={{ p: 3, mt: 3 }}>
          <Typography variant="body1" color="text.secondary" paragraph>
            This is a placeholder for the shopping cart. In the future, this will display:
          </Typography>
          <Box component="ul" sx={{ pl: 3, mb: 3 }}>
            <li>List of items in the cart</li>
            <li>Product images and names</li>
            <li>Quantity controls (increase/decrease)</li>
            <li>Individual item prices</li>
            <li>Subtotal for each item</li>
            <li>Total cart value</li>
            <li>Checkout button</li>
            <li>Remove item functionality</li>
          </Box>

          <Divider sx={{ my: 3 }} />

          <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
            <Typography variant="h5" component="div">
              Total: $0.00
            </Typography>
            <Box sx={{ display: 'flex', gap: 2 }}>
              <Button
                variant="outlined"
                component={RouterLink}
                to="/products"
              >
                Continue Shopping
              </Button>
              <Button
                variant="contained"
                color="primary"
                disabled
              >
                Proceed to Checkout
              </Button>
            </Box>
          </Box>
        </Paper>
      </Box>
    </Container>
  );
}

export default Cart;

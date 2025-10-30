import React from 'react';
import { Container, Typography, Paper, Box, Button } from '@mui/material';

function Cart() {
  // Placeholder - will be replaced with actual cart state from API
  const cartItems = [];
  const cartTotal = 0;

  return (
    <Container maxWidth="md">
      <Typography variant="h4" component="h1" gutterBottom>
        Shopping Cart
      </Typography>
      <Paper elevation={3} sx={{ p: 4, mt: 3 }}>
        {cartItems.length === 0 ? (
          <Box sx={{ textAlign: 'center', py: 4 }}>
            <Typography variant="h6" color="text.secondary">
              Your cart is empty
            </Typography>
            <Typography variant="body2" color="text.secondary" sx={{ mt: 2 }}>
              Start shopping to add items to your cart!
            </Typography>
          </Box>
        ) : (
          <Box>
            <Typography variant="body1">
              This is a placeholder for cart items. In the future, this will display:
            </Typography>
            <Box component="ul" sx={{ mt: 2 }}>
              <Typography component="li">List of cart items with images</Typography>
              <Typography component="li">Quantity controls</Typography>
              <Typography component="li">Individual item prices</Typography>
              <Typography component="li">Remove item buttons</Typography>
              <Typography component="li">Cart subtotal and total</Typography>
            </Box>
            <Box sx={{ mt: 3, pt: 3, borderTop: 1, borderColor: 'divider' }}>
              <Typography variant="h6" align="right">
                Total: ${cartTotal.toFixed(2)}
              </Typography>
              <Button
                variant="contained"
                color="primary"
                fullWidth
                sx={{ mt: 2 }}
              >
                Proceed to Checkout
              </Button>
            </Box>
          </Box>
        )}
      </Paper>
    </Container>
  );
}

export default Cart;

import React from 'react';
import { Link as RouterLink } from 'react-router-dom';
import {
  Container,
  Typography,
  Button,
  Box,
  Paper,
} from '@mui/material';
import ShoppingCartIcon from '@mui/icons-material/ShoppingCart';

function Cart() {
  // Placeholder - will be replaced with actual cart data from API
  const cartItems = [];

  return (
    <Container maxWidth="md">
      <Typography variant="h4" component="h1" gutterBottom sx={{ mb: 4 }}>
        Shopping Cart
      </Typography>

      {cartItems.length === 0 ? (
        <Paper
          elevation={2}
          sx={{
            p: 6,
            textAlign: 'center',
            backgroundColor: 'grey.50',
          }}
        >
          <ShoppingCartIcon
            sx={{ fontSize: 80, color: 'text.secondary', mb: 2 }}
          />
          <Typography variant="h6" color="text.secondary" gutterBottom>
            Your cart is empty
          </Typography>
          <Typography variant="body1" color="text.secondary" paragraph>
            Add some products to get started!
          </Typography>
          <Button
            variant="contained"
            color="primary"
            component={RouterLink}
            to="/products"
            sx={{ mt: 2 }}
          >
            Browse Products
          </Button>
        </Paper>
      ) : (
        <Box>
          {/* Cart items will be displayed here */}
          <Typography variant="body1" color="text.secondary">
            Cart items will be displayed here when cart functionality is
            implemented.
          </Typography>

          <Box sx={{ mt: 4, display: 'flex', justifyContent: 'space-between' }}>
            <Typography variant="h6">Total:</Typography>
            <Typography variant="h6" color="primary">
              $0.00
            </Typography>
          </Box>

          <Box sx={{ mt: 3 }}>
            <Button
              variant="contained"
              color="primary"
              size="large"
              fullWidth
            >
              Proceed to Checkout
            </Button>
          </Box>
        </Box>
      )}
    </Container>
  );
}

export default Cart;

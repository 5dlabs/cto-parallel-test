import React from 'react';
import { Link } from 'react-router-dom';
import {
  Container,
  Typography,
  Button,
  Box,
  Paper,
  Divider,
} from '@mui/material';
import ShoppingCartIcon from '@mui/icons-material/ShoppingCart';

function Cart() {
  // TODO: Connect to cart API to display actual cart items

  return (
    <Container maxWidth="md">
      <Typography variant="h3" component="h1" gutterBottom>
        Shopping Cart
      </Typography>

      <Paper elevation={2} sx={{ p: 4, mt: 3 }}>
        <Box
          sx={{
            display: 'flex',
            flexDirection: 'column',
            alignItems: 'center',
            py: 4,
          }}
        >
          <ShoppingCartIcon sx={{ fontSize: 60, color: 'text.secondary', mb: 2 }} />
          <Typography variant="h6" color="text.secondary" gutterBottom>
            Your cart is empty
          </Typography>
          <Typography variant="body2" color="text.secondary" paragraph>
            This is a placeholder for the shopping cart page.
          </Typography>
          <Typography variant="body2" color="text.secondary" paragraph>
            Future implementation will display cart items with quantities,
            prices, and checkout options.
          </Typography>
        </Box>

        <Divider sx={{ my: 3 }} />

        <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
          <Typography variant="h5">
            Total: $0.00
          </Typography>
          <Box sx={{ display: 'flex', gap: 2 }}>
            <Button
              variant="outlined"
              component={Link}
              to="/products"
            >
              Continue Shopping
            </Button>
            <Button
              variant="contained"
              color="primary"
              disabled
            >
              Checkout
            </Button>
          </Box>
        </Box>
      </Paper>
    </Container>
  );
}

export default Cart;

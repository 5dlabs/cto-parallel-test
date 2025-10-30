import React from 'react';
import { Link as RouterLink } from 'react-router-dom';
import { Container, Typography, Button, Box, Paper } from '@mui/material';
import ShoppingCartIcon from '@mui/icons-material/ShoppingCart';

function Cart() {
  // TODO: Fetch cart items from API
  // TODO: Implement cart item management (update quantity, remove items)

  return (
    <Container maxWidth="lg" sx={{ mt: 4, mb: 4 }}>
      <Typography variant="h3" component="h1" gutterBottom>
        Shopping Cart
      </Typography>

      <Paper sx={{ p: 3, mt: 3 }}>
        <Box
          sx={{
            display: 'flex',
            flexDirection: 'column',
            alignItems: 'center',
            py: 5,
          }}
        >
          <ShoppingCartIcon sx={{ fontSize: 80, color: 'text.secondary', mb: 2 }} />
          <Typography variant="h6" color="text.secondary" gutterBottom>
            Your cart is empty
          </Typography>
          <Typography variant="body2" color="text.secondary" paragraph>
            This is a placeholder for cart items list.
          </Typography>
          <Typography variant="body2" color="text.secondary" paragraph>
            Future implementation will display items from the cart API.
          </Typography>
          <Button
            variant="contained"
            component={RouterLink}
            to="/products"
            sx={{ mt: 2 }}
          >
            Continue Shopping
          </Button>
        </Box>
      </Paper>
    </Container>
  );
}

export default Cart;

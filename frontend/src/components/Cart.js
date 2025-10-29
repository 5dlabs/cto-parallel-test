import React from 'react';
import { Link } from 'react-router-dom';
import Container from '@mui/material/Container';
import Typography from '@mui/material/Typography';
import Button from '@mui/material/Button';
import Box from '@mui/material/Box';
import Paper from '@mui/material/Paper';

function Cart() {
  return (
    <Container maxWidth="md">
      <Typography variant="h3" component="h1" gutterBottom>
        Shopping Cart
      </Typography>
      <Paper elevation={3} sx={{ p: 4, mt: 2 }}>
        <Typography variant="body1" color="text.secondary" paragraph>
          Your cart is currently empty. This is a placeholder page that will 
          display cart items fetched from the API in the future.
        </Typography>
        <Box sx={{ mt: 3 }}>
          <Button
            variant="contained"
            component={Link}
            to="/products"
          >
            Continue Shopping
          </Button>
        </Box>
      </Paper>
    </Container>
  );
}

export default Cart;

import { Container, Typography, Box, Button, Paper, Divider } from '@mui/material';
import { Link } from 'react-router-dom';

function Cart() {
  // TODO: Fetch cart items from API

  return (
    <Container maxWidth="md">
      <Typography variant="h3" component="h1" gutterBottom sx={{ mb: 4 }}>
        Shopping Cart
      </Typography>
      <Paper elevation={3} sx={{ p: 4 }}>
        <Typography variant="body1" color="text.secondary" paragraph>
          This is a placeholder for the shopping cart.
        </Typography>
        <Typography variant="body1" color="text.secondary" paragraph>
          Future implementation will display:
        </Typography>
        <Box component="ul" sx={{ pl: 2 }}>
          <Typography component="li" variant="body2" color="text.secondary">
            List of items in the cart
          </Typography>
          <Typography component="li" variant="body2" color="text.secondary">
            Quantity controls for each item
          </Typography>
          <Typography component="li" variant="body2" color="text.secondary">
            Price calculations and total
          </Typography>
          <Typography component="li" variant="body2" color="text.secondary">
            Checkout button
          </Typography>
        </Box>
        <Divider sx={{ my: 3 }} />
        <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
          <Typography variant="h5">
            Total: $0.00
          </Typography>
          <Box>
            <Button
              variant="outlined"
              color="primary"
              component={Link}
              to="/products"
              sx={{ mr: 2 }}
            >
              Continue Shopping
            </Button>
            <Button variant="contained" color="primary" disabled>
              Checkout
            </Button>
          </Box>
        </Box>
      </Paper>
    </Container>
  );
}

export default Cart;

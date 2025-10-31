import Container from '@mui/material/Container';
import Typography from '@mui/material/Typography';
import Stack from '@mui/material/Stack';
import Divider from '@mui/material/Divider';

const Cart = () => (
  <Container maxWidth="md" sx={{ mt: 4 }}>
    <Stack spacing={3}>
      <Typography variant="h4" component="h1" fontWeight={600}>
        Shopping Cart
      </Typography>
      <Divider />
      <Typography variant="body1" color="text.secondary">
        Your cart is currently empty. Items added to your cart will appear here for quick checkout.
      </Typography>
    </Stack>
  </Container>
);

export default Cart;

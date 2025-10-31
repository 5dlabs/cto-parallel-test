import Container from '@mui/material/Container';
import Typography from '@mui/material/Typography';
import Button from '@mui/material/Button';
import Stack from '@mui/material/Stack';
import { Link as RouterLink } from 'react-router-dom';

const HomePage = () => (
  <Container maxWidth="md" sx={{ textAlign: 'center', mt: 8 }}>
    <Stack spacing={3} alignItems="center">
      <Typography variant="h3" component="h1" color="primary" fontWeight={700}>
        Welcome to ShopSmart
      </Typography>
      <Typography variant="h6" color="textSecondary">
        Discover exclusive deals and the latest products in our curated collection.
      </Typography>
      <Button
        component={RouterLink}
        to="/products"
        variant="contained"
        color="secondary"
        size="large"
      >
        Shop Now
      </Button>
    </Stack>
  </Container>
);

export default HomePage;

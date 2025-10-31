import Container from '@mui/material/Container';
import Typography from '@mui/material/Typography';
import Stack from '@mui/material/Stack';
import Button from '@mui/material/Button';
import { useParams, Link as RouterLink } from 'react-router-dom';

const ProductDetail = () => {
  const { id } = useParams();

  return (
    <Container maxWidth="md" sx={{ mt: 4 }}>
      <Stack spacing={3}>
        <Typography variant="h4" component="h1" fontWeight={600}>
          Product Detail - ID: {id}
        </Typography>
        <Typography variant="body1" color="text.secondary">
          Detailed product information will be available soon. Stay tuned for specifications, reviews, and availability updates.
        </Typography>
        <Button component={RouterLink} to="/products" variant="outlined" color="primary" sx={{ alignSelf: 'flex-start' }}>
          Back to Products
        </Button>
      </Stack>
    </Container>
  );
};

export default ProductDetail;

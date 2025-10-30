import { Container, Typography, Box, Button, Paper } from '@mui/material';
import { useParams, Link } from 'react-router-dom';

function ProductDetail() {
  const { id } = useParams();

  // TODO: Fetch product details from API using id

  return (
    <Container maxWidth="md">
      <Paper elevation={3} sx={{ p: 4, mt: 4 }}>
        <Typography variant="h3" component="h1" gutterBottom>
          Product Detail - ID: {id}
        </Typography>
        <Typography variant="body1" color="text.secondary" paragraph>
          This is a placeholder for the product detail page.
        </Typography>
        <Typography variant="body1" color="text.secondary" paragraph>
          Future implementation will fetch and display detailed product information from the backend API.
        </Typography>
        <Box sx={{ mt: 3 }}>
          <Button
            variant="contained"
            color="primary"
            component={Link}
            to="/products"
            sx={{ mr: 2 }}
          >
            Back to Products
          </Button>
          <Button variant="outlined" color="primary">
            Add to Cart
          </Button>
        </Box>
      </Paper>
    </Container>
  );
}

export default ProductDetail;

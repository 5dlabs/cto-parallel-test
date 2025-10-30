import React from 'react';
import { useParams, Link } from 'react-router-dom';
import { Container, Typography, Button, Box } from '@mui/material';
import ArrowBackIcon from '@mui/icons-material/ArrowBack';

function ProductDetail() {
  const { id } = useParams();

  // Placeholder - will fetch product details from API later
  return (
    <Container maxWidth="md">
      <Box sx={{ my: 4 }}>
        <Button
          component={Link}
          to="/products"
          startIcon={<ArrowBackIcon />}
          sx={{ mb: 3 }}
        >
          Back to Products
        </Button>
        <Typography variant="h3" component="h1" gutterBottom>
          Product Detail - ID: {id}
        </Typography>
        <Typography variant="body1" color="text.secondary" paragraph>
          This is a placeholder for product details. In the future, this page will display
          complete product information fetched from the API including images, description,
          specifications, reviews, and purchasing options.
        </Typography>
        <Box sx={{ mt: 3 }}>
          <Button variant="contained" color="primary" size="large">
            Add to Cart
          </Button>
        </Box>
      </Box>
    </Container>
  );
}

export default ProductDetail;

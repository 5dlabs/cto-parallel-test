import React from 'react';
import { useParams, Link } from 'react-router-dom';
import { Container, Typography, Button, Box, Paper } from '@mui/material';
import ArrowBackIcon from '@mui/icons-material/ArrowBack';

function ProductDetail() {
  const { id } = useParams();

  // TODO: Fetch product details from API using the id

  return (
    <Container maxWidth="md">
      <Box sx={{ mb: 3 }}>
        <Button
          component={Link}
          to="/products"
          startIcon={<ArrowBackIcon />}
          variant="outlined"
        >
          Back to Products
        </Button>
      </Box>

      <Paper elevation={2} sx={{ p: 4 }}>
        <Typography variant="h3" component="h1" gutterBottom>
          Product Detail - ID: {id}
        </Typography>

        <Typography variant="body1" color="text.secondary" paragraph>
          This is a placeholder for the product detail page.
        </Typography>

        <Typography variant="body1" color="text.secondary" paragraph>
          Future implementation will fetch and display full product information
          including images, detailed description, specifications, reviews, and
          purchase options.
        </Typography>

        <Box sx={{ mt: 4 }}>
          <Button variant="contained" color="primary" size="large">
            Add to Cart
          </Button>
        </Box>
      </Paper>
    </Container>
  );
}

export default ProductDetail;

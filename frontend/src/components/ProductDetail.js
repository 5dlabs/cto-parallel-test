import React from 'react';
import { useParams, Link as RouterLink } from 'react-router-dom';
import {
  Container,
  Typography,
  Button,
  Box,
  Paper,
} from '@mui/material';
import ArrowBackIcon from '@mui/icons-material/ArrowBack';

function ProductDetail() {
  const { id } = useParams();

  // TODO: Fetch product details from API using the id
  // This is a placeholder component

  return (
    <Container maxWidth="md">
      <Box sx={{ my: 4 }}>
        <Button
          component={RouterLink}
          to="/products"
          startIcon={<ArrowBackIcon />}
          sx={{ mb: 3 }}
        >
          Back to Products
        </Button>

        <Paper elevation={3} sx={{ p: 4 }}>
          <Typography variant="h3" component="h1" gutterBottom>
            Product Detail - ID: {id}
          </Typography>

          <Typography variant="body1" color="text.secondary" paragraph>
            This is a placeholder for product details. In a full implementation,
            this page would display comprehensive information about the product
            including images, full description, specifications, reviews, and
            purchase options.
          </Typography>

          <Typography variant="body2" color="text.secondary" sx={{ mt: 3 }}>
            TODO: Fetch and display product details from the API using product ID: {id}
          </Typography>

          <Box sx={{ mt: 4 }}>
            <Button variant="contained" size="large" disabled>
              Add to Cart
            </Button>
          </Box>
        </Paper>
      </Box>
    </Container>
  );
}

export default ProductDetail;

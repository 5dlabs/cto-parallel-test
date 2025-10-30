import React from 'react';
import { useParams, Link as RouterLink } from 'react-router-dom';
import { Container, Typography, Button, Box } from '@mui/material';
import ArrowBackIcon from '@mui/icons-material/ArrowBack';

function ProductDetail() {
  const { id } = useParams();

  // TODO: Fetch product details from API using the id
  // For now, display placeholder content

  return (
    <Container maxWidth="lg" sx={{ mt: 4, mb: 4 }}>
      <Button
        component={RouterLink}
        to="/products"
        startIcon={<ArrowBackIcon />}
        sx={{ mb: 2 }}
      >
        Back to Products
      </Button>

      <Box sx={{ textAlign: 'center', py: 5 }}>
        <Typography variant="h3" component="h1" gutterBottom>
          Product Detail - ID: {id}
        </Typography>
        <Typography variant="body1" color="text.secondary" paragraph>
          This is a placeholder for full product information.
        </Typography>
        <Typography variant="body2" color="text.secondary">
          Future implementation will fetch and display complete product details from the API.
        </Typography>
      </Box>
    </Container>
  );
}

export default ProductDetail;

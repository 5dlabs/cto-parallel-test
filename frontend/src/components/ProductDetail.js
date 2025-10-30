import React from 'react';
import { useParams } from 'react-router-dom';
import { Container, Typography, Paper, Box } from '@mui/material';

function ProductDetail() {
  const { id } = useParams();

  // Placeholder - will be replaced with API call to fetch product details
  return (
    <Container maxWidth="md">
      <Paper elevation={3} sx={{ p: 4, mt: 4 }}>
        <Typography variant="h4" component="h1" gutterBottom>
          Product Detail - ID: {id}
        </Typography>
        <Box sx={{ mt: 3 }}>
          <Typography variant="body1" color="text.secondary">
            This is a placeholder for product details. In the future, this will display:
          </Typography>
          <Box component="ul" sx={{ mt: 2 }}>
            <Typography component="li">Product images</Typography>
            <Typography component="li">Detailed description</Typography>
            <Typography component="li">Price and availability</Typography>
            <Typography component="li">Add to cart functionality</Typography>
            <Typography component="li">Customer reviews</Typography>
          </Box>
        </Box>
      </Paper>
    </Container>
  );
}

export default ProductDetail;

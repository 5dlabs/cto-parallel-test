import React from 'react';
import { useParams, Link } from 'react-router-dom';
import Container from '@mui/material/Container';
import Typography from '@mui/material/Typography';
import Button from '@mui/material/Button';
import Box from '@mui/material/Box';
import Paper from '@mui/material/Paper';

function ProductDetail() {
  const { id } = useParams();

  return (
    <Container maxWidth="md">
      <Paper elevation={3} sx={{ p: 4, mt: 4 }}>
        <Typography variant="h3" component="h1" gutterBottom>
          Product Detail - ID: {id}
        </Typography>
        <Typography variant="body1" color="text.secondary" paragraph>
          This is a placeholder for product details. In a full implementation, this would fetch
          product information from the API and display complete product details including:
        </Typography>
        <Box component="ul" sx={{ mb: 3 }}>
          <li>Product name and description</li>
          <li>Price and availability</li>
          <li>Product images</li>
          <li>Customer reviews</li>
          <li>Add to cart functionality</li>
        </Box>
        <Box sx={{ display: 'flex', gap: 2, mt: 4 }}>
          <Button variant="contained" color="primary">
            Add to Cart
          </Button>
          <Button variant="outlined" component={Link} to="/products">
            Back to Products
          </Button>
        </Box>
      </Paper>
    </Container>
  );
}

export default ProductDetail;

import React from 'react';
import { useParams, Link as RouterLink } from 'react-router-dom';
import Container from '@mui/material/Container';
import Typography from '@mui/material/Typography';
import Box from '@mui/material/Box';
import Button from '@mui/material/Button';
import Paper from '@mui/material/Paper';

function ProductDetail() {
  const { id } = useParams();

  // TODO: Fetch product details from API using the id

  return (
    <Container maxWidth="md">
      <Box sx={{ my: 4 }}>
        <Paper elevation={3} sx={{ p: 4 }}>
          <Typography variant="h3" component="h1" gutterBottom>
            Product Detail - ID: {id}
          </Typography>
          <Typography variant="body1" color="text.secondary" paragraph>
            This is a placeholder for the product detail page. In the future, this will display:
          </Typography>
          <Box component="ul" sx={{ pl: 3, mb: 3 }}>
            <li>Product name and description</li>
            <li>High-resolution product images</li>
            <li>Price and availability</li>
            <li>Customer reviews and ratings</li>
            <li>Shipping information</li>
            <li>Add to cart functionality</li>
          </Box>
          <Box sx={{ display: 'flex', gap: 2, mt: 4 }}>
            <Button
              variant="contained"
              color="primary"
              onClick={() => console.log(`Adding product ${id} to cart`)}
            >
              Add to Cart
            </Button>
            <Button
              variant="outlined"
              component={RouterLink}
              to="/products"
            >
              Back to Products
            </Button>
          </Box>
        </Paper>
      </Box>
    </Container>
  );
}

export default ProductDetail;

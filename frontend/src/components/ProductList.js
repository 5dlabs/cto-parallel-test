import React from 'react';
import { Link } from 'react-router-dom';
import {
  Container,
  Grid,
  Card,
  CardContent,
  CardMedia,
  Typography,
  Button,
  Box,
} from '@mui/material';

function ProductList() {
  // Placeholder product data - will be replaced with API call later
  const products = [
    {
      id: 1,
      name: 'Product 1',
      price: 19.99,
      description: 'This is a great product with amazing features.',
    },
    {
      id: 2,
      name: 'Product 2',
      price: 29.99,
      description: 'An excellent choice for your needs.',
    },
    {
      id: 3,
      name: 'Product 3',
      price: 39.99,
      description: 'Premium quality at an affordable price.',
    },
  ];

  const handleAddToCart = (product) => {
    // TODO: Implement add to cart functionality
    console.log('Add to cart:', product);
  };

  return (
    <Container maxWidth="lg">
      <Typography variant="h4" component="h1" gutterBottom sx={{ mb: 4 }}>
        Products
      </Typography>
      <Grid container spacing={4}>
        {products.map((product) => (
          <Grid item key={product.id} xs={12} sm={6} md={4}>
            <Card sx={{ height: '100%', display: 'flex', flexDirection: 'column' }}>
              <CardMedia
                component="div"
                sx={{
                  pt: '56.25%', // 16:9 aspect ratio
                  backgroundColor: 'grey.300',
                }}
                title={product.name}
              />
              <CardContent sx={{ flexGrow: 1 }}>
                <Typography gutterBottom variant="h5" component="h2">
                  {product.name}
                </Typography>
                <Typography variant="h6" color="primary" gutterBottom>
                  ${product.price.toFixed(2)}
                </Typography>
                <Typography variant="body2" color="text.secondary">
                  {product.description}
                </Typography>
              </CardContent>
              <Box sx={{ p: 2, pt: 0 }}>
                <Button
                  fullWidth
                  variant="outlined"
                  component={Link}
                  to={`/products/${product.id}`}
                  sx={{ mb: 1 }}
                >
                  View Details
                </Button>
                <Button
                  fullWidth
                  variant="contained"
                  color="primary"
                  onClick={() => handleAddToCart(product)}
                >
                  Add to Cart
                </Button>
              </Box>
            </Card>
          </Grid>
        ))}
      </Grid>
    </Container>
  );
}

export default ProductList;

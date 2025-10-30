import React from 'react';
import { Link as RouterLink } from 'react-router-dom';
import {
  Container,
  Grid,
  Card,
  CardContent,
  CardMedia,
  CardActions,
  Typography,
  Button,
} from '@mui/material';

function ProductList() {
  // TODO: Replace with API call to backend
  const products = [
    {
      id: 1,
      name: 'Product 1',
      price: 19.99,
      description: 'High-quality product with excellent features',
    },
    {
      id: 2,
      name: 'Product 2',
      price: 29.99,
      description: 'Premium product designed for professionals',
    },
    {
      id: 3,
      name: 'Product 3',
      price: 39.99,
      description: 'Luxury product with advanced functionality',
    },
  ];

  const handleAddToCart = (productId) => {
    // TODO: Implement add to cart functionality
    console.log(`Add product ${productId} to cart`);
  };

  return (
    <Container maxWidth="lg" sx={{ mt: 4, mb: 4 }}>
      <Typography variant="h3" component="h1" gutterBottom>
        Products
      </Typography>
      <Grid container spacing={4}>
        {products.map((product) => (
          <Grid item key={product.id} xs={12} sm={6} md={4}>
            <Card
              sx={{
                height: '100%',
                display: 'flex',
                flexDirection: 'column',
              }}
            >
              <CardMedia
                component="div"
                sx={{
                  pt: '56.25%', // 16:9 aspect ratio
                  backgroundColor: 'grey.300',
                }}
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
              <CardActions>
                <Button
                  size="small"
                  component={RouterLink}
                  to={`/products/${product.id}`}
                >
                  View Details
                </Button>
                <Button
                  size="small"
                  color="primary"
                  variant="contained"
                  onClick={() => handleAddToCart(product.id)}
                >
                  Add to Cart
                </Button>
              </CardActions>
            </Card>
          </Grid>
        ))}
      </Grid>
    </Container>
  );
}

export default ProductList;

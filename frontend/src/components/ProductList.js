import React from 'react';
import { useNavigate } from 'react-router-dom';
import {
  Container,
  Typography,
  Grid,
  Card,
  CardContent,
  CardMedia,
  CardActions,
  Button,
  Box,
} from '@mui/material';

function ProductList() {
  const navigate = useNavigate();

  // Placeholder product data - will be replaced with API call
  const products = [
    {
      id: 1,
      name: 'Product 1',
      price: 19.99,
      description: 'High-quality product with amazing features',
    },
    {
      id: 2,
      name: 'Product 2',
      price: 29.99,
      description: 'Premium product for discerning customers',
    },
    {
      id: 3,
      name: 'Product 3',
      price: 39.99,
      description: 'Top-of-the-line product with exceptional value',
    },
  ];

  const handleAddToCart = (productId) => {
    // Placeholder - will be replaced with actual cart functionality
    console.log(`Adding product ${productId} to cart`);
  };

  return (
    <Container maxWidth="lg">
      <Typography variant="h3" component="h1" gutterBottom sx={{ mt: 4, mb: 4 }}>
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
                  bgcolor: 'grey.300',
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
                  onClick={() => navigate(`/products/${product.id}`)}
                >
                  View Details
                </Button>
                <Button
                  size="small"
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

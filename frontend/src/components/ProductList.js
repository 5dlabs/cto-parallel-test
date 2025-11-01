import React, { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import {
  Container,
  Grid,
  Card,
  CardMedia,
  CardContent,
  CardActions,
  Typography,
  Button,
  Box,
  CircularProgress,
} from '@mui/material';
import ShoppingCartIcon from '@mui/icons-material/ShoppingCart';

function ProductList() {
  const navigate = useNavigate();
  const [products, setProducts] = useState([]);
  const [loading, setLoading] = useState(true);

  // Mock products - will be replaced with API call
  useEffect(() => {
    // Simulate API call
    setTimeout(() => {
      setProducts([
        {
          id: 1,
          name: 'Premium Headphones',
          description: 'High-quality wireless headphones with noise cancellation',
          price: 299.99,
          image: 'https://via.placeholder.com/300x200?text=Headphones',
          inventory_count: 15,
        },
        {
          id: 2,
          name: 'Smart Watch',
          description: 'Feature-rich smartwatch with fitness tracking',
          price: 399.99,
          image: 'https://via.placeholder.com/300x200?text=Smart+Watch',
          inventory_count: 8,
        },
        {
          id: 3,
          name: 'Laptop Stand',
          description: 'Ergonomic aluminum laptop stand',
          price: 79.99,
          image: 'https://via.placeholder.com/300x200?text=Laptop+Stand',
          inventory_count: 25,
        },
        {
          id: 4,
          name: 'Wireless Mouse',
          description: 'Precision wireless mouse with ergonomic design',
          price: 49.99,
          image: 'https://via.placeholder.com/300x200?text=Wireless+Mouse',
          inventory_count: 40,
        },
        {
          id: 5,
          name: 'Mechanical Keyboard',
          description: 'RGB mechanical keyboard with customizable keys',
          price: 149.99,
          image: 'https://via.placeholder.com/300x200?text=Keyboard',
          inventory_count: 12,
        },
        {
          id: 6,
          name: 'USB-C Hub',
          description: '7-in-1 USB-C hub with multiple ports',
          price: 59.99,
          image: 'https://via.placeholder.com/300x200?text=USB-C+Hub',
          inventory_count: 30,
        },
      ]);
      setLoading(false);
    }, 500);
  }, []);

  const handleAddToCart = (product) => {
    // Will be implemented with cart functionality
    console.log('Adding to cart:', product);
  };

  const handleViewDetails = (productId) => {
    navigate(`/products/${productId}`);
  };

  if (loading) {
    return (
      <Box
        sx={{
          display: 'flex',
          justifyContent: 'center',
          alignItems: 'center',
          minHeight: '60vh',
        }}
      >
        <CircularProgress />
      </Box>
    );
  }

  return (
    <Container maxWidth="lg">
      <Typography variant="h3" component="h1" gutterBottom sx={{ mb: 4 }}>
        Our Products
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
                component="img"
                height="200"
                image={product.image}
                alt={product.name}
                sx={{ objectFit: 'cover' }}
              />
              <CardContent sx={{ flexGrow: 1 }}>
                <Typography gutterBottom variant="h5" component="h2">
                  {product.name}
                </Typography>
                <Typography variant="body2" color="text.secondary" paragraph>
                  {product.description}
                </Typography>
                <Typography variant="h6" color="primary">
                  ${product.price.toFixed(2)}
                </Typography>
                <Typography variant="body2" color="text.secondary">
                  {product.inventory_count > 0
                    ? `In Stock: ${product.inventory_count}`
                    : 'Out of Stock'}
                </Typography>
              </CardContent>
              <CardActions>
                <Button
                  size="small"
                  onClick={() => handleViewDetails(product.id)}
                >
                  View Details
                </Button>
                <Button
                  size="small"
                  variant="contained"
                  startIcon={<ShoppingCartIcon />}
                  onClick={() => handleAddToCart(product)}
                  disabled={product.inventory_count === 0}
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

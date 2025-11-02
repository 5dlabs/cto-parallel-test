import React, { useState, useEffect } from 'react';
import { Link as RouterLink } from 'react-router-dom';
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

// Mock products - in real app would fetch from API
const mockProducts = [
  {
    id: 1,
    name: 'Wireless Headphones',
    description: 'Premium noise-canceling wireless headphones with 30-hour battery life',
    price: 299.99,
    image: 'https://via.placeholder.com/300x200/1976d2/ffffff?text=Headphones',
    inventory_count: 50,
  },
  {
    id: 2,
    name: 'Smart Watch',
    description: 'Fitness tracking smartwatch with heart rate monitor and GPS',
    price: 399.99,
    image: 'https://via.placeholder.com/300x200/dc004e/ffffff?text=Smart+Watch',
    inventory_count: 30,
  },
  {
    id: 3,
    name: 'Laptop Backpack',
    description: 'Durable laptop backpack with multiple compartments and USB charging port',
    price: 79.99,
    image: 'https://via.placeholder.com/300x200/388e3c/ffffff?text=Backpack',
    inventory_count: 100,
  },
  {
    id: 4,
    name: 'Mechanical Keyboard',
    description: 'RGB mechanical gaming keyboard with Cherry MX switches',
    price: 149.99,
    image: 'https://via.placeholder.com/300x200/f57c00/ffffff?text=Keyboard',
    inventory_count: 45,
  },
  {
    id: 5,
    name: 'Wireless Mouse',
    description: 'Ergonomic wireless mouse with precision tracking',
    price: 59.99,
    image: 'https://via.placeholder.com/300x200/7b1fa2/ffffff?text=Mouse',
    inventory_count: 75,
  },
  {
    id: 6,
    name: '4K Webcam',
    description: 'Ultra HD 4K webcam with auto-focus and noise reduction',
    price: 199.99,
    image: 'https://via.placeholder.com/300x200/0288d1/ffffff?text=Webcam',
    inventory_count: 25,
  },
  {
    id: 7,
    name: 'USB-C Hub',
    description: '7-in-1 USB-C hub with HDMI, USB 3.0, and SD card reader',
    price: 49.99,
    image: 'https://via.placeholder.com/300x200/303f9f/ffffff?text=USB+Hub',
    inventory_count: 60,
  },
  {
    id: 8,
    name: 'Portable SSD',
    description: '1TB portable SSD with USB-C interface and 540MB/s transfer speed',
    price: 129.99,
    image: 'https://via.placeholder.com/300x200/c2185b/ffffff?text=SSD',
    inventory_count: 40,
  },
];

function ProductList() {
  const [products, setProducts] = useState([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    // Simulate API call
    const fetchProducts = async () => {
      setLoading(true);
      // In real app: const response = await axios.get('/api/products');
      // Simulate network delay
      await new Promise((resolve) => setTimeout(resolve, 500));
      setProducts(mockProducts);
      setLoading(false);
    };

    fetchProducts();
  }, []);

  const handleAddToCart = (productId) => {
    // In real app, this would add to cart state/context
    console.log('Add to cart:', productId);
    alert('Product added to cart!');
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
    <Container maxWidth="lg" sx={{ py: 8 }}>
      <Typography
        component="h1"
        variant="h3"
        color="text.primary"
        gutterBottom
        sx={{ mb: 4, fontWeight: 'bold' }}
      >
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
                transition: 'transform 0.2s',
                '&:hover': {
                  transform: 'scale(1.03)',
                },
              }}
            >
              <CardMedia
                component="img"
                height="200"
                image={product.image}
                alt={product.name}
              />
              <CardContent sx={{ flexGrow: 1 }}>
                <Typography gutterBottom variant="h5" component="h2">
                  {product.name}
                </Typography>
                <Typography variant="body2" color="text.secondary" paragraph>
                  {product.description}
                </Typography>
                <Typography variant="h6" color="primary" sx={{ fontWeight: 'bold' }}>
                  ${product.price.toFixed(2)}
                </Typography>
                <Typography variant="caption" color="text.secondary">
                  {product.inventory_count > 0
                    ? `${product.inventory_count} in stock`
                    : 'Out of stock'}
                </Typography>
              </CardContent>
              <CardActions sx={{ p: 2, pt: 0 }}>
                <Button
                  component={RouterLink}
                  to={`/products/${product.id}`}
                  size="small"
                  variant="outlined"
                  fullWidth
                >
                  View Details
                </Button>
                <Button
                  size="small"
                  variant="contained"
                  fullWidth
                  startIcon={<ShoppingCartIcon />}
                  onClick={() => handleAddToCart(product.id)}
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

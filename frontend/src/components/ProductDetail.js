import React, { useState, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import {
  Container,
  Grid,
  Typography,
  Button,
  Box,
  Card,
  CardMedia,
  CircularProgress,
  TextField,
} from '@mui/material';
import ShoppingCartIcon from '@mui/icons-material/ShoppingCart';
import ArrowBackIcon from '@mui/icons-material/ArrowBack';

function ProductDetail() {
  const { id } = useParams();
  const navigate = useNavigate();
  const [product, setProduct] = useState(null);
  const [loading, setLoading] = useState(true);
  const [quantity, setQuantity] = useState(1);

  // Mock product data - will be replaced with API call
  useEffect(() => {
    // Simulate API call
    setTimeout(() => {
      const mockProducts = {
        1: {
          id: 1,
          name: 'Premium Headphones',
          description: 'High-quality wireless headphones with active noise cancellation. Features include 30-hour battery life, premium sound quality, and comfortable over-ear design.',
          price: 299.99,
          image: 'https://via.placeholder.com/600x400?text=Headphones',
          inventory_count: 15,
          specs: [
            'Battery Life: 30 hours',
            'Bluetooth 5.0',
            'Active Noise Cancellation',
            'Premium leather ear cups',
          ],
        },
        2: {
          id: 2,
          name: 'Smart Watch',
          description: 'Feature-rich smartwatch with comprehensive fitness tracking and health monitoring capabilities.',
          price: 399.99,
          image: 'https://via.placeholder.com/600x400?text=Smart+Watch',
          inventory_count: 8,
          specs: [
            'Heart rate monitor',
            'GPS tracking',
            'Water resistant',
            '7-day battery life',
          ],
        },
      };

      setProduct(mockProducts[id] || null);
      setLoading(false);
    }, 500);
  }, [id]);

  const handleAddToCart = () => {
    // Will be implemented with cart functionality
    console.log('Adding to cart:', { product, quantity });
  };

  const handleQuantityChange = (event) => {
    const value = parseInt(event.target.value, 10);
    if (value > 0 && value <= product.inventory_count) {
      setQuantity(value);
    }
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

  if (!product) {
    return (
      <Container maxWidth="lg">
        <Typography variant="h4" sx={{ my: 4 }}>
          Product not found
        </Typography>
        <Button
          startIcon={<ArrowBackIcon />}
          onClick={() => navigate('/products')}
        >
          Back to Products
        </Button>
      </Container>
    );
  }

  return (
    <Container maxWidth="lg">
      <Button
        startIcon={<ArrowBackIcon />}
        onClick={() => navigate('/products')}
        sx={{ mb: 3 }}
      >
        Back to Products
      </Button>

      <Grid container spacing={4}>
        <Grid item xs={12} md={6}>
          <Card>
            <CardMedia
              component="img"
              image={product.image}
              alt={product.name}
              sx={{ width: '100%', height: 'auto' }}
            />
          </Card>
        </Grid>

        <Grid item xs={12} md={6}>
          <Typography variant="h3" component="h1" gutterBottom>
            {product.name}
          </Typography>

          <Typography variant="h4" color="primary" gutterBottom>
            ${product.price.toFixed(2)}
          </Typography>

          <Typography variant="body1" paragraph sx={{ my: 3 }}>
            {product.description}
          </Typography>

          {product.specs && (
            <Box sx={{ my: 3 }}>
              <Typography variant="h6" gutterBottom>
                Specifications:
              </Typography>
              <ul>
                {product.specs.map((spec, index) => (
                  <li key={index}>
                    <Typography variant="body2">{spec}</Typography>
                  </li>
                ))}
              </ul>
            </Box>
          )}

          <Typography variant="body2" color="text.secondary" gutterBottom>
            {product.inventory_count > 0
              ? `In Stock: ${product.inventory_count} available`
              : 'Out of Stock'}
          </Typography>

          <Box sx={{ display: 'flex', gap: 2, alignItems: 'center', my: 3 }}>
            <TextField
              type="number"
              label="Quantity"
              value={quantity}
              onChange={handleQuantityChange}
              inputProps={{
                min: 1,
                max: product.inventory_count,
              }}
              sx={{ width: 120 }}
            />
            <Button
              variant="contained"
              size="large"
              startIcon={<ShoppingCartIcon />}
              onClick={handleAddToCart}
              disabled={product.inventory_count === 0}
            >
              Add to Cart
            </Button>
          </Box>
        </Grid>
      </Grid>
    </Container>
  );
}

export default ProductDetail;

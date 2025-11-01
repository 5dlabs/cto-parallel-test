import React, { useState, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import {
  Container,
  Grid,
  Card,
  CardMedia,
  Typography,
  Button,
  Box,
  CircularProgress,
  Divider,
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

  useEffect(() => {
    // Simulate API call with mock data
    // In production: axios.get(`/api/products/${id}`)
    setTimeout(() => {
      const mockProducts = {
        1: {
          id: 1,
          name: 'Wireless Headphones',
          description: 'High-quality wireless headphones with noise cancellation',
          price: 129.99,
          inventory_count: 50,
          image: 'https://via.placeholder.com/600x400/1976d2/ffffff?text=Headphones',
          features: [
            'Active Noise Cancellation',
            '30-hour battery life',
            'Bluetooth 5.0',
            'Comfortable over-ear design',
          ],
        },
        2: {
          id: 2,
          name: 'Smart Watch',
          description: 'Feature-rich smartwatch with health tracking',
          price: 299.99,
          inventory_count: 30,
          image: 'https://via.placeholder.com/600x400/dc004e/ffffff?text=Smart+Watch',
          features: [
            'Heart rate monitoring',
            'GPS tracking',
            'Water resistant',
            'Sleep tracking',
          ],
        },
        3: {
          id: 3,
          name: 'Laptop Stand',
          description: 'Ergonomic laptop stand for better posture',
          price: 49.99,
          inventory_count: 100,
          image: 'https://via.placeholder.com/600x400/43a047/ffffff?text=Laptop+Stand',
          features: [
            'Adjustable height',
            'Aluminum construction',
            'Cable management',
            'Supports up to 15" laptops',
          ],
        },
        4: {
          id: 4,
          name: 'Mechanical Keyboard',
          description: 'RGB mechanical keyboard with Cherry MX switches',
          price: 159.99,
          inventory_count: 25,
          image: 'https://via.placeholder.com/600x400/f57c00/ffffff?text=Keyboard',
          features: [
            'Cherry MX Red switches',
            'RGB backlighting',
            'Programmable keys',
            'USB-C connection',
          ],
        },
        5: {
          id: 5,
          name: 'Wireless Mouse',
          description: 'Precision wireless mouse with ergonomic design',
          price: 39.99,
          inventory_count: 75,
          image: 'https://via.placeholder.com/600x400/8e24aa/ffffff?text=Mouse',
          features: [
            '2400 DPI sensor',
            'Ergonomic design',
            'Long battery life',
            '6 programmable buttons',
          ],
        },
        6: {
          id: 6,
          name: 'USB-C Hub',
          description: 'Multi-port USB-C hub with HDMI and USB 3.0',
          price: 69.99,
          inventory_count: 60,
          image: 'https://via.placeholder.com/600x400/00acc1/ffffff?text=USB+Hub',
          features: [
            'HDMI 4K output',
            '3x USB 3.0 ports',
            'SD card reader',
            'Compact aluminum design',
          ],
        },
      };

      const foundProduct = mockProducts[id];
      setProduct(foundProduct || null);
      setLoading(false);
    }, 300);
  }, [id]);

  const handleAddToCart = () => {
    if (!product) return;

    const cart = JSON.parse(localStorage.getItem('cart') || '[]');
    const existingItem = cart.find(item => item.id === product.id);

    if (existingItem) {
      existingItem.quantity += quantity;
    } else {
      cart.push({ ...product, quantity });
    }

    localStorage.setItem('cart', JSON.stringify(cart));
    window.dispatchEvent(new Event('cartUpdated'));
    navigate('/cart');
  };

  const handleQuantityChange = (e) => {
    const value = parseInt(e.target.value, 10);
    if (value >= 1 && value <= product.inventory_count) {
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
        <Box sx={{ textAlign: 'center', py: 8 }}>
          <Typography variant="h4" gutterBottom>
            Product Not Found
          </Typography>
          <Button
            variant="contained"
            startIcon={<ArrowBackIcon />}
            onClick={() => navigate('/products')}
            sx={{ mt: 2 }}
          >
            Back to Products
          </Button>
        </Box>
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
              sx={{ height: 400, objectFit: 'cover' }}
            />
          </Card>
        </Grid>

        <Grid item xs={12} md={6}>
          <Typography variant="h3" component="h1" gutterBottom sx={{ fontWeight: 700 }}>
            {product.name}
          </Typography>

          <Typography variant="h4" color="primary" gutterBottom sx={{ fontWeight: 600 }}>
            ${product.price.toFixed(2)}
          </Typography>

          <Typography variant="body1" color="text.secondary" paragraph>
            {product.description}
          </Typography>

          <Divider sx={{ my: 3 }} />

          <Typography variant="h6" gutterBottom>
            Features:
          </Typography>
          <Box component="ul" sx={{ pl: 2 }}>
            {product.features.map((feature, index) => (
              <Typography component="li" variant="body1" key={index} sx={{ mb: 1 }}>
                {feature}
              </Typography>
            ))}
          </Box>

          <Divider sx={{ my: 3 }} />

          <Typography variant="body1" color="text.secondary" gutterBottom>
            {product.inventory_count > 0
              ? `In Stock (${product.inventory_count} available)`
              : 'Out of Stock'}
          </Typography>

          <Box sx={{ display: 'flex', alignItems: 'center', gap: 2, mt: 3 }}>
            <TextField
              type="number"
              label="Quantity"
              value={quantity}
              onChange={handleQuantityChange}
              inputProps={{
                min: 1,
                max: product.inventory_count,
              }}
              sx={{ width: 100 }}
            />

            <Button
              variant="contained"
              size="large"
              startIcon={<ShoppingCartIcon />}
              onClick={handleAddToCart}
              disabled={product.inventory_count === 0}
              sx={{ flex: 1 }}
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

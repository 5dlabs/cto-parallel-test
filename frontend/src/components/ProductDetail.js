import React, { useState, useEffect } from 'react';
import {
  Container,
  Grid,
  Card,
  CardMedia,
  Typography,
  Button,
  Box,
  CircularProgress,
  Alert,
  TextField,
  Divider
} from '@mui/material';
import {
  ShoppingCart as ShoppingCartIcon,
  ArrowBack as ArrowBackIcon
} from '@mui/icons-material';
import { useNavigate, useParams } from 'react-router-dom';
import axios from 'axios';

function ProductDetail() {
  const { id } = useParams();
  const navigate = useNavigate();
  const [product, setProduct] = useState(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const [quantity, setQuantity] = useState(1);

  useEffect(() => {
    fetchProductDetail();
  }, [id]);

  const fetchProductDetail = async () => {
    try {
      setLoading(true);
      const apiUrl = process.env.REACT_APP_API_URL || 'http://localhost:8080/api';
      const response = await axios.get(`${apiUrl}/products/${id}`);
      setProduct(response.data);
      setError(null);
    } catch (err) {
      // Placeholder data for development
      console.warn('API not available, using placeholder data:', err.message);
      const placeholderProducts = {
        1: {
          id: 1,
          name: 'Premium Wireless Headphones',
          description: 'Experience crystal-clear audio with our premium wireless headphones. Features active noise cancellation, 30-hour battery life, and premium comfort padding.',
          price: 199.99,
          inventory_count: 50,
          image: 'https://via.placeholder.com/600x400?text=Headphones'
        },
        2: {
          id: 2,
          name: 'Smart Watch Pro',
          description: 'Stay connected and track your fitness goals with our advanced smartwatch. Water-resistant, GPS-enabled, and compatible with all major smartphones.',
          price: 299.99,
          inventory_count: 30,
          image: 'https://via.placeholder.com/600x400?text=Smart+Watch'
        },
        3: {
          id: 3,
          name: 'Laptop Stand',
          description: 'Ergonomic aluminum laptop stand designed to improve posture and reduce neck strain. Compatible with all laptop sizes from 10 to 17 inches.',
          price: 49.99,
          inventory_count: 100,
          image: 'https://via.placeholder.com/600x400?text=Laptop+Stand'
        }
      };
      setProduct(placeholderProducts[id] || placeholderProducts[1]);
      setError(null);
    } finally {
      setLoading(false);
    }
  };

  const handleAddToCart = async () => {
    try {
      const token = localStorage.getItem('token');
      if (!token) {
        navigate('/login');
        return;
      }

      const apiUrl = process.env.REACT_APP_API_URL || 'http://localhost:8080/api';
      await axios.post(
        `${apiUrl}/cart/add`,
        { product_id: product.id, quantity },
        { headers: { Authorization: `Bearer ${token}` } }
      );

      alert(`Added ${quantity} item(s) to cart!`);
    } catch (err) {
      console.error('Failed to add to cart:', err);
      alert('Failed to add product to cart. Please try again.');
    }
  };

  const handleQuantityChange = (event) => {
    const value = parseInt(event.target.value, 10);
    if (value >= 1 && value <= product.inventory_count) {
      setQuantity(value);
    }
  };

  if (loading) {
    return (
      <Container maxWidth="lg">
        <Box display="flex" justifyContent="center" alignItems="center" minHeight="400px">
          <CircularProgress />
        </Box>
      </Container>
    );
  }

  if (error || !product) {
    return (
      <Container maxWidth="lg">
        <Alert severity="error" sx={{ mt: 4 }}>
          {error || 'Product not found'}
        </Alert>
        <Button
          startIcon={<ArrowBackIcon />}
          onClick={() => navigate('/products')}
          sx={{ mt: 2 }}
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
              image={product.image || 'https://via.placeholder.com/600x400?text=Product'}
              alt={product.name}
              sx={{ height: 'auto', maxHeight: 500 }}
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

          <Divider sx={{ my: 2 }} />

          <Typography variant="body1" paragraph>
            {product.description}
          </Typography>

          <Box sx={{ my: 3 }}>
            <Typography variant="subtitle1" gutterBottom>
              Availability:
            </Typography>
            <Typography
              variant="body1"
              color={product.inventory_count > 0 ? 'success.main' : 'error.main'}
            >
              {product.inventory_count > 0
                ? `In Stock (${product.inventory_count} available)`
                : 'Out of Stock'}
            </Typography>
          </Box>

          {product.inventory_count > 0 && (
            <>
              <Box sx={{ my: 3 }}>
                <Typography variant="subtitle1" gutterBottom>
                  Quantity:
                </Typography>
                <TextField
                  type="number"
                  value={quantity}
                  onChange={handleQuantityChange}
                  inputProps={{
                    min: 1,
                    max: product.inventory_count
                  }}
                  sx={{ width: 100 }}
                />
              </Box>

              <Button
                variant="contained"
                size="large"
                startIcon={<ShoppingCartIcon />}
                onClick={handleAddToCart}
                fullWidth
                sx={{ mt: 2 }}
              >
                Add to Cart
              </Button>
            </>
          )}
        </Grid>
      </Grid>
    </Container>
  );
}

export default ProductDetail;

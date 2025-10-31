import React, { useState, useEffect } from 'react';
import {
  Container,
  Grid,
  Card,
  CardContent,
  CardMedia,
  CardActions,
  Typography,
  Button,
  Box,
  CircularProgress,
  Alert,
  TextField,
  InputAdornment
} from '@mui/material';
import {
  Search as SearchIcon,
  ShoppingCart as ShoppingCartIcon
} from '@mui/icons-material';
import { useNavigate } from 'react-router-dom';
import axios from 'axios';

function ProductList() {
  const navigate = useNavigate();
  const [products, setProducts] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const [searchTerm, setSearchTerm] = useState('');

  useEffect(() => {
    fetchProducts();
  }, []);

  const fetchProducts = async () => {
    try {
      setLoading(true);
      // When backend is ready, replace with actual API endpoint
      const apiUrl = process.env.REACT_APP_API_URL || 'http://localhost:8080/api';
      const response = await axios.get(`${apiUrl}/products`);
      setProducts(response.data);
      setError(null);
    } catch (err) {
      // For now, use placeholder data if API is not available
      console.warn('API not available, using placeholder data:', err.message);
      setProducts([
        {
          id: 1,
          name: 'Premium Wireless Headphones',
          description: 'High-quality audio with noise cancellation',
          price: 199.99,
          inventory_count: 50,
          image: 'https://via.placeholder.com/300x200?text=Headphones'
        },
        {
          id: 2,
          name: 'Smart Watch Pro',
          description: 'Track your fitness and stay connected',
          price: 299.99,
          inventory_count: 30,
          image: 'https://via.placeholder.com/300x200?text=Smart+Watch'
        },
        {
          id: 3,
          name: 'Laptop Stand',
          description: 'Ergonomic aluminum stand for laptops',
          price: 49.99,
          inventory_count: 100,
          image: 'https://via.placeholder.com/300x200?text=Laptop+Stand'
        },
        {
          id: 4,
          name: 'Mechanical Keyboard',
          description: 'RGB backlit gaming keyboard',
          price: 129.99,
          inventory_count: 75,
          image: 'https://via.placeholder.com/300x200?text=Keyboard'
        },
        {
          id: 5,
          name: 'Wireless Mouse',
          description: 'Ergonomic design with precision tracking',
          price: 59.99,
          inventory_count: 120,
          image: 'https://via.placeholder.com/300x200?text=Mouse'
        },
        {
          id: 6,
          name: 'USB-C Hub',
          description: 'Multi-port adapter for modern laptops',
          price: 79.99,
          inventory_count: 60,
          image: 'https://via.placeholder.com/300x200?text=USB+Hub'
        }
      ]);
      setError(null);
    } finally {
      setLoading(false);
    }
  };

  const handleAddToCart = async (productId) => {
    try {
      const token = localStorage.getItem('token');
      if (!token) {
        navigate('/login');
        return;
      }

      const apiUrl = process.env.REACT_APP_API_URL || 'http://localhost:8080/api';
      await axios.post(
        `${apiUrl}/cart/add`,
        { product_id: productId, quantity: 1 },
        { headers: { Authorization: `Bearer ${token}` } }
      );

      alert('Product added to cart!');
    } catch (err) {
      console.error('Failed to add to cart:', err);
      alert('Failed to add product to cart. Please try again.');
    }
  };

  const filteredProducts = products.filter(product =>
    product.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
    product.description.toLowerCase().includes(searchTerm.toLowerCase())
  );

  if (loading) {
    return (
      <Container maxWidth="lg">
        <Box display="flex" justifyContent="center" alignItems="center" minHeight="400px">
          <CircularProgress />
        </Box>
      </Container>
    );
  }

  return (
    <Container maxWidth="lg">
      <Box sx={{ mb: 4 }}>
        <Typography variant="h3" component="h1" gutterBottom>
          Our Products
        </Typography>
        <TextField
          fullWidth
          variant="outlined"
          placeholder="Search products..."
          value={searchTerm}
          onChange={(e) => setSearchTerm(e.target.value)}
          InputProps={{
            startAdornment: (
              <InputAdornment position="start">
                <SearchIcon />
              </InputAdornment>
            ),
          }}
          sx={{ mt: 2 }}
        />
      </Box>

      {error && (
        <Alert severity="error" sx={{ mb: 3 }}>
          {error}
        </Alert>
      )}

      <Grid container spacing={4}>
        {filteredProducts.map((product) => (
          <Grid item key={product.id} xs={12} sm={6} md={4}>
            <Card
              sx={{
                height: '100%',
                display: 'flex',
                flexDirection: 'column',
                transition: 'transform 0.2s',
                '&:hover': {
                  transform: 'scale(1.03)',
                  boxShadow: 6
                }
              }}
            >
              <CardMedia
                component="img"
                height="200"
                image={product.image || 'https://via.placeholder.com/300x200?text=Product'}
                alt={product.name}
                sx={{ cursor: 'pointer' }}
                onClick={() => navigate(`/products/${product.id}`)}
              />
              <CardContent sx={{ flexGrow: 1 }}>
                <Typography
                  gutterBottom
                  variant="h5"
                  component="h2"
                  sx={{ cursor: 'pointer' }}
                  onClick={() => navigate(`/products/${product.id}`)}
                >
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
                  fullWidth
                  variant="contained"
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

      {filteredProducts.length === 0 && !loading && (
        <Box sx={{ textAlign: 'center', py: 8 }}>
          <Typography variant="h5" color="text.secondary">
            No products found
          </Typography>
        </Box>
      )}
    </Container>
  );
}

export default ProductList;

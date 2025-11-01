import React, { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
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
  TextField,
  InputAdornment,
} from '@mui/material';
import SearchIcon from '@mui/icons-material/Search';
import ShoppingCartIcon from '@mui/icons-material/ShoppingCart';

function ProductList() {
  const navigate = useNavigate();
  const [products, setProducts] = useState([]);
  const [loading, setLoading] = useState(true);
  const [searchTerm, setSearchTerm] = useState('');

  useEffect(() => {
    // Simulate API call with mock data
    // In production, this would be: axios.get('/api/products')
    setTimeout(() => {
      const mockProducts = [
        {
          id: 1,
          name: 'Wireless Headphones',
          description: 'High-quality wireless headphones with noise cancellation',
          price: 129.99,
          inventory_count: 50,
          image: 'https://via.placeholder.com/300x200/1976d2/ffffff?text=Headphones',
        },
        {
          id: 2,
          name: 'Smart Watch',
          description: 'Feature-rich smartwatch with health tracking',
          price: 299.99,
          inventory_count: 30,
          image: 'https://via.placeholder.com/300x200/dc004e/ffffff?text=Smart+Watch',
        },
        {
          id: 3,
          name: 'Laptop Stand',
          description: 'Ergonomic laptop stand for better posture',
          price: 49.99,
          inventory_count: 100,
          image: 'https://via.placeholder.com/300x200/43a047/ffffff?text=Laptop+Stand',
        },
        {
          id: 4,
          name: 'Mechanical Keyboard',
          description: 'RGB mechanical keyboard with Cherry MX switches',
          price: 159.99,
          inventory_count: 25,
          image: 'https://via.placeholder.com/300x200/f57c00/ffffff?text=Keyboard',
        },
        {
          id: 5,
          name: 'Wireless Mouse',
          description: 'Precision wireless mouse with ergonomic design',
          price: 39.99,
          inventory_count: 75,
          image: 'https://via.placeholder.com/300x200/8e24aa/ffffff?text=Mouse',
        },
        {
          id: 6,
          name: 'USB-C Hub',
          description: 'Multi-port USB-C hub with HDMI and USB 3.0',
          price: 69.99,
          inventory_count: 60,
          image: 'https://via.placeholder.com/300x200/00acc1/ffffff?text=USB+Hub',
        },
      ];
      setProducts(mockProducts);
      setLoading(false);
    }, 500);
  }, []);

  const handleAddToCart = (product) => {
    const cart = JSON.parse(localStorage.getItem('cart') || '[]');
    const existingItem = cart.find(item => item.id === product.id);

    if (existingItem) {
      existingItem.quantity += 1;
    } else {
      cart.push({ ...product, quantity: 1 });
    }

    localStorage.setItem('cart', JSON.stringify(cart));
    // Trigger a custom event to update header badge
    window.dispatchEvent(new Event('cartUpdated'));
  };

  const filteredProducts = products.filter(product =>
    product.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
    product.description.toLowerCase().includes(searchTerm.toLowerCase())
  );

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
      <Typography variant="h3" component="h1" gutterBottom sx={{ fontWeight: 700 }}>
        Our Products
      </Typography>

      <Box sx={{ mb: 4 }}>
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
        />
      </Box>

      <Grid container spacing={3}>
        {filteredProducts.map((product) => (
          <Grid item xs={12} sm={6} md={4} key={product.id}>
            <Card
              sx={{
                height: '100%',
                display: 'flex',
                flexDirection: 'column',
                transition: 'transform 0.2s',
                '&:hover': {
                  transform: 'translateY(-4px)',
                  boxShadow: 3,
                },
              }}
            >
              <CardMedia
                component="img"
                height="200"
                image={product.image}
                alt={product.name}
                sx={{ cursor: 'pointer' }}
                onClick={() => navigate(`/products/${product.id}`)}
              />
              <CardContent sx={{ flexGrow: 1 }}>
                <Typography
                  gutterBottom
                  variant="h6"
                  component="h2"
                  sx={{ cursor: 'pointer' }}
                  onClick={() => navigate(`/products/${product.id}`)}
                >
                  {product.name}
                </Typography>
                <Typography variant="body2" color="text.secondary" paragraph>
                  {product.description}
                </Typography>
                <Typography variant="h6" color="primary" sx={{ fontWeight: 600 }}>
                  ${product.price.toFixed(2)}
                </Typography>
                <Typography variant="body2" color="text.secondary">
                  {product.inventory_count > 0
                    ? `In Stock (${product.inventory_count})`
                    : 'Out of Stock'}
                </Typography>
              </CardContent>
              <CardActions sx={{ p: 2 }}>
                <Button
                  fullWidth
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

      {filteredProducts.length === 0 && (
        <Box sx={{ textAlign: 'center', py: 8 }}>
          <Typography variant="h6" color="text.secondary">
            No products found matching your search.
          </Typography>
        </Box>
      )}
    </Container>
  );
}

export default ProductList;

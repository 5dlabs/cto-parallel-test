import React, { useState, useEffect } from 'react';
import {
  Container,
  Typography,
  Box,
  Card,
  CardContent,
  Grid,
  Button,
  IconButton,
  TextField,
  Divider,
  Alert,
  CircularProgress
} from '@mui/material';
import {
  Delete as DeleteIcon,
  ShoppingCart as ShoppingCartIcon,
  ArrowBack as ArrowBackIcon
} from '@mui/icons-material';
import { useNavigate } from 'react-router-dom';
import axios from 'axios';

function Cart() {
  const navigate = useNavigate();
  const [cartItems, setCartItems] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  useEffect(() => {
    fetchCart();
  }, []);

  const fetchCart = async () => {
    try {
      setLoading(true);
      const token = localStorage.getItem('token');

      if (!token) {
        setLoading(false);
        return;
      }

      const apiUrl = process.env.REACT_APP_API_URL || 'http://localhost:8080/api';
      const response = await axios.get(`${apiUrl}/cart`, {
        headers: { Authorization: `Bearer ${token}` }
      });

      setCartItems(response.data.items || []);
      setError(null);
    } catch (err) {
      // Placeholder data for development
      console.warn('API not available, using placeholder data:', err.message);
      setCartItems([
        {
          id: 1,
          product_id: 1,
          product_name: 'Premium Wireless Headphones',
          price: 199.99,
          quantity: 2,
          image: 'https://via.placeholder.com/150x100?text=Headphones'
        },
        {
          id: 2,
          product_id: 3,
          product_name: 'Laptop Stand',
          price: 49.99,
          quantity: 1,
          image: 'https://via.placeholder.com/150x100?text=Laptop+Stand'
        }
      ]);
      setError(null);
    } finally {
      setLoading(false);
    }
  };

  const handleRemoveItem = async (itemId) => {
    try {
      const token = localStorage.getItem('token');
      if (!token) {
        navigate('/login');
        return;
      }

      const apiUrl = process.env.REACT_APP_API_URL || 'http://localhost:8080/api';
      await axios.delete(`${apiUrl}/cart/remove/${itemId}`, {
        headers: { Authorization: `Bearer ${token}` }
      });

      setCartItems(cartItems.filter(item => item.id !== itemId));
    } catch (err) {
      console.error('Failed to remove item:', err);
      // For placeholder data, just remove from state
      setCartItems(cartItems.filter(item => item.id !== itemId));
    }
  };

  const handleQuantityChange = (itemId, newQuantity) => {
    if (newQuantity < 1) return;

    setCartItems(cartItems.map(item =>
      item.id === itemId ? { ...item, quantity: newQuantity } : item
    ));
  };

  const handleClearCart = async () => {
    try {
      const token = localStorage.getItem('token');
      if (!token) {
        navigate('/login');
        return;
      }

      const apiUrl = process.env.REACT_APP_API_URL || 'http://localhost:8080/api';
      await axios.post(`${apiUrl}/cart/clear`, {}, {
        headers: { Authorization: `Bearer ${token}` }
      });

      setCartItems([]);
    } catch (err) {
      console.error('Failed to clear cart:', err);
      // For placeholder data, just clear state
      setCartItems([]);
    }
  };

  const calculateTotal = () => {
    return cartItems.reduce((total, item) => total + (item.price * item.quantity), 0);
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

  const token = localStorage.getItem('token');

  if (!token) {
    return (
      <Container maxWidth="lg">
        <Box sx={{ textAlign: 'center', py: 8 }}>
          <ShoppingCartIcon sx={{ fontSize: 80, color: 'text.secondary', mb: 2 }} />
          <Typography variant="h4" gutterBottom>
            Please Login to View Cart
          </Typography>
          <Typography variant="body1" color="text.secondary" paragraph>
            You need to be logged in to access your shopping cart.
          </Typography>
          <Button
            variant="contained"
            onClick={() => navigate('/login')}
            sx={{ mt: 2 }}
          >
            Login
          </Button>
        </Box>
      </Container>
    );
  }

  if (cartItems.length === 0) {
    return (
      <Container maxWidth="lg">
        <Box sx={{ textAlign: 'center', py: 8 }}>
          <ShoppingCartIcon sx={{ fontSize: 80, color: 'text.secondary', mb: 2 }} />
          <Typography variant="h4" gutterBottom>
            Your Cart is Empty
          </Typography>
          <Typography variant="body1" color="text.secondary" paragraph>
            Start adding products to your cart!
          </Typography>
          <Button
            variant="contained"
            onClick={() => navigate('/products')}
            sx={{ mt: 2 }}
          >
            Browse Products
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
        Continue Shopping
      </Button>

      <Typography variant="h3" component="h1" gutterBottom>
        Shopping Cart
      </Typography>

      {error && (
        <Alert severity="error" sx={{ mb: 3 }}>
          {error}
        </Alert>
      )}

      <Grid container spacing={4}>
        <Grid item xs={12} md={8}>
          {cartItems.map((item) => (
            <Card key={item.id} sx={{ mb: 2 }}>
              <CardContent>
                <Grid container spacing={2} alignItems="center">
                  <Grid item xs={12} sm={3}>
                    <Box
                      component="img"
                      src={item.image || 'https://via.placeholder.com/150x100?text=Product'}
                      alt={item.product_name}
                      sx={{
                        width: '100%',
                        height: 'auto',
                        borderRadius: 1
                      }}
                    />
                  </Grid>
                  <Grid item xs={12} sm={4}>
                    <Typography variant="h6">
                      {item.product_name}
                    </Typography>
                    <Typography variant="body2" color="text.secondary">
                      ${item.price.toFixed(2)} each
                    </Typography>
                  </Grid>
                  <Grid item xs={12} sm={3}>
                    <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
                      <Typography variant="body2">Qty:</Typography>
                      <TextField
                        type="number"
                        value={item.quantity}
                        onChange={(e) => handleQuantityChange(item.id, parseInt(e.target.value, 10))}
                        inputProps={{ min: 1 }}
                        size="small"
                        sx={{ width: 80 }}
                      />
                    </Box>
                    <Typography variant="subtitle1" sx={{ mt: 1 }}>
                      Subtotal: ${(item.price * item.quantity).toFixed(2)}
                    </Typography>
                  </Grid>
                  <Grid item xs={12} sm={2}>
                    <IconButton
                      color="error"
                      onClick={() => handleRemoveItem(item.id)}
                      aria-label="remove item"
                    >
                      <DeleteIcon />
                    </IconButton>
                  </Grid>
                </Grid>
              </CardContent>
            </Card>
          ))}

          <Button
            variant="outlined"
            color="error"
            onClick={handleClearCart}
            sx={{ mt: 2 }}
          >
            Clear Cart
          </Button>
        </Grid>

        <Grid item xs={12} md={4}>
          <Card>
            <CardContent>
              <Typography variant="h5" gutterBottom>
                Order Summary
              </Typography>
              <Divider sx={{ my: 2 }} />

              <Box sx={{ mb: 2 }}>
                <Box sx={{ display: 'flex', justifyContent: 'space-between', mb: 1 }}>
                  <Typography variant="body1">Subtotal:</Typography>
                  <Typography variant="body1">
                    ${calculateTotal().toFixed(2)}
                  </Typography>
                </Box>
                <Box sx={{ display: 'flex', justifyContent: 'space-between', mb: 1 }}>
                  <Typography variant="body1">Shipping:</Typography>
                  <Typography variant="body1">
                    {calculateTotal() > 100 ? 'FREE' : '$10.00'}
                  </Typography>
                </Box>
                <Box sx={{ display: 'flex', justifyContent: 'space-between', mb: 1 }}>
                  <Typography variant="body1">Tax (10%):</Typography>
                  <Typography variant="body1">
                    ${(calculateTotal() * 0.1).toFixed(2)}
                  </Typography>
                </Box>
              </Box>

              <Divider sx={{ my: 2 }} />

              <Box sx={{ display: 'flex', justifyContent: 'space-between', mb: 3 }}>
                <Typography variant="h6">Total:</Typography>
                <Typography variant="h6" color="primary">
                  ${(calculateTotal() * 1.1 + (calculateTotal() > 100 ? 0 : 10)).toFixed(2)}
                </Typography>
              </Box>

              <Button
                variant="contained"
                fullWidth
                size="large"
                onClick={() => alert('Checkout functionality coming soon!')}
              >
                Proceed to Checkout
              </Button>

              {calculateTotal() > 100 && (
                <Alert severity="success" sx={{ mt: 2 }}>
                  You qualify for free shipping!
                </Alert>
              )}
            </CardContent>
          </Card>
        </Grid>
      </Grid>
    </Container>
  );
}

export default Cart;

import React, { useState, useEffect } from 'react';
import { Link as RouterLink } from 'react-router-dom';
import {
  Container,
  Typography,
  Button,
  Box,
  Paper,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  IconButton,
  Divider,
  CircularProgress,
  Alert,
} from '@mui/material';
import DeleteIcon from '@mui/icons-material/Delete';
import ShoppingCartIcon from '@mui/icons-material/ShoppingCart';
import { cartAPI } from '../services/api';

function Cart() {
  const [cartItems, setCartItems] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const [removing, setRemoving] = useState({});

  useEffect(() => {
    loadCart();
  }, []);

  const loadCart = async () => {
    try {
      setLoading(true);
      setError(null);
      const response = await cartAPI.getCart();
      setCartItems(response.data.items || []);
    } catch (err) {
      console.error('Failed to load cart:', err);
      setError(err.response?.data?.message || 'Failed to load cart. Please try again later.');
    } finally {
      setLoading(false);
    }
  };

  const handleRemoveItem = async (productId) => {
    try {
      setRemoving({ ...removing, [productId]: true });
      await cartAPI.removeItem(productId);
      setCartItems(cartItems.filter(item => item.product_id !== productId));
    } catch (err) {
      console.error('Failed to remove item:', err);
      alert(err.response?.data?.message || 'Failed to remove item. Please try again.');
    } finally {
      setRemoving({ ...removing, [productId]: false });
    }
  };

  const handleClearCart = async () => {
    if (!window.confirm('Are you sure you want to clear your cart?')) {
      return;
    }
    try {
      await cartAPI.clearCart();
      setCartItems([]);
    } catch (err) {
      console.error('Failed to clear cart:', err);
      alert(err.response?.data?.message || 'Failed to clear cart. Please try again.');
    }
  };

  const subtotal = cartItems.reduce((sum, item) => sum + Number(item.price) * item.quantity, 0);
  const tax = subtotal * 0.1;
  const total = subtotal + tax;

  if (loading) {
    return (
      <Container maxWidth="lg" sx={{ mt: 4, mb: 4, display: 'flex', justifyContent: 'center' }}>
        <CircularProgress />
      </Container>
    );
  }

  if (error) {
    return (
      <Container maxWidth="lg" sx={{ mt: 4, mb: 4 }}>
        <Alert
          severity="error"
          action={
            <Button color="inherit" size="small" onClick={loadCart}>
              Retry
            </Button>
          }
        >
          {error}
        </Alert>
      </Container>
    );
  }

  return (
    <Container maxWidth="lg" sx={{ mt: 4, mb: 4 }}>
      <Typography variant="h3" component="h1" gutterBottom>
        Shopping Cart
      </Typography>

      {cartItems.length === 0 ? (
        <Paper elevation={3} sx={{ p: 4, textAlign: 'center' }}>
          <ShoppingCartIcon sx={{ fontSize: 80, color: 'text.secondary', mb: 2 }} />
          <Typography variant="h5" color="text.secondary" paragraph>
            Your cart is empty
          </Typography>
          <Button variant="contained" component={RouterLink} to="/products">
            Continue Shopping
          </Button>
        </Paper>
      ) : (
        <Box>
          <TableContainer component={Paper} elevation={3}>
            <Table>
              <TableHead>
                <TableRow>
                  <TableCell>Product</TableCell>
                  <TableCell align="right">Price</TableCell>
                  <TableCell align="center">Quantity</TableCell>
                  <TableCell align="right">Total</TableCell>
                  <TableCell align="center">Actions</TableCell>
                </TableRow>
              </TableHead>
              <TableBody>
                {cartItems.map((item) => (
                  <TableRow key={item.product_id}>
                    <TableCell>
                      <Box sx={{ display: 'flex', alignItems: 'center', gap: 2 }}>
                        <img
                          src={item.image_url || `https://via.placeholder.com/80?text=${encodeURIComponent(item.name)}`}
                          alt={item.name}
                          style={{ width: 60, height: 60, objectFit: 'cover', borderRadius: 4 }}
                        />
                        <Typography>{item.name}</Typography>
                      </Box>
                    </TableCell>
                    <TableCell align="right">${Number(item.price).toFixed(2)}</TableCell>
                    <TableCell align="center">{item.quantity}</TableCell>
                    <TableCell align="right">${(Number(item.price) * item.quantity).toFixed(2)}</TableCell>
                    <TableCell align="center">
                      <IconButton
                        color="error"
                        aria-label="delete"
                        onClick={() => handleRemoveItem(item.product_id)}
                        disabled={removing[item.product_id]}
                      >
                        <DeleteIcon />
                      </IconButton>
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </TableContainer>

          <Paper elevation={3} sx={{ mt: 3, p: 3 }}>
            <Box sx={{ display: 'flex', flexDirection: 'column', gap: 1 }}>
              <Box sx={{ display: 'flex', justifyContent: 'space-between' }}>
                <Typography variant="body1">Subtotal:</Typography>
                <Typography variant="body1">${subtotal.toFixed(2)}</Typography>
              </Box>
              <Box sx={{ display: 'flex', justifyContent: 'space-between' }}>
                <Typography variant="body1">Tax (10%):</Typography>
                <Typography variant="body1">${tax.toFixed(2)}</Typography>
              </Box>
              <Divider sx={{ my: 1 }} />
              <Box sx={{ display: 'flex', justifyContent: 'space-between' }}>
                <Typography variant="h5">Total:</Typography>
                <Typography variant="h5" color="primary">
                  ${total.toFixed(2)}
                </Typography>
              </Box>
              <Box sx={{ mt: 2, display: 'flex', gap: 2 }}>
                <Button variant="contained" size="large" fullWidth>
                  Proceed to Checkout
                </Button>
                <Button variant="outlined" color="error" onClick={handleClearCart}>
                  Clear Cart
                </Button>
              </Box>
            </Box>
          </Paper>
        </Box>
      )}
    </Container>
  );
}

export default Cart;

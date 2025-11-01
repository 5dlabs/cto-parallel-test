import React, { useState, useEffect } from 'react';
import { Link as RouterLink } from 'react-router-dom';
import {
  Container,
  Typography,
  Grid,
  Card,
  CardContent,
  CardMedia,
  CardActions,
  Button,
  CircularProgress,
  Alert,
  Box,
} from '@mui/material';
import AddShoppingCartIcon from '@mui/icons-material/AddShoppingCart';
import { productAPI, cartAPI } from '../services/api';

function ProductList() {
  const [products, setProducts] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const [addingToCart, setAddingToCart] = useState({});

  useEffect(() => {
    loadProducts();
  }, []);

  const loadProducts = async () => {
    try {
      setLoading(true);
      setError(null);
      const response = await productAPI.getAll();
      setProducts(response.data);
    } catch (err) {
      console.error('Failed to load products:', err);
      setError(err.response?.data?.message || 'Failed to load products. Please try again later.');
    } finally {
      setLoading(false);
    }
  };

  const handleAddToCart = async (productId) => {
    try {
      setAddingToCart({ ...addingToCart, [productId]: true });
      await cartAPI.addItem(productId, 1);
      // Success - could show a snackbar notification
    } catch (err) {
      console.error('Failed to add to cart:', err);
      alert(err.response?.data?.message || 'Failed to add to cart. Please try again.');
    } finally {
      setAddingToCart({ ...addingToCart, [productId]: false });
    }
  };

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
            <Button color="inherit" size="small" onClick={loadProducts}>
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
        Products
      </Typography>
      {products.length === 0 ? (
        <Alert severity="info">No products available at this time.</Alert>
      ) : (
        <Grid container spacing={3}>
          {products.map((product) => (
            <Grid item key={product.id} xs={12} sm={6} md={4}>
              <Card sx={{ height: '100%', display: 'flex', flexDirection: 'column' }}>
                <CardMedia
                  component="img"
                  height="200"
                  image={product.image_url || `https://via.placeholder.com/300x200?text=${encodeURIComponent(product.name)}`}
                  alt={product.name}
                />
                <CardContent sx={{ flexGrow: 1 }}>
                  <Typography gutterBottom variant="h5" component="h2">
                    {product.name}
                  </Typography>
                  <Typography variant="h6" color="primary">
                    ${Number(product.price).toFixed(2)}
                  </Typography>
                  {product.inventory_count !== undefined && (
                    <Typography variant="body2" color="text.secondary">
                      In stock: {product.inventory_count}
                    </Typography>
                  )}
                </CardContent>
                <CardActions>
                  <Button
                    size="small"
                    component={RouterLink}
                    to={`/products/${product.id}`}
                  >
                    View Details
                  </Button>
                  <Button
                    size="small"
                    startIcon={<AddShoppingCartIcon />}
                    color="primary"
                    onClick={() => handleAddToCart(product.id)}
                    disabled={addingToCart[product.id] || product.inventory_count === 0}
                  >
                    {addingToCart[product.id] ? 'Adding...' : 'Add to Cart'}
                  </Button>
                </CardActions>
              </Card>
            </Grid>
          ))}
        </Grid>
      )}
    </Container>
  );
}

export default ProductList;

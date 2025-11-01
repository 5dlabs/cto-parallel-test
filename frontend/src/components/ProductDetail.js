import React, { useState, useEffect } from 'react';
import { useParams, Link as RouterLink, useNavigate } from 'react-router-dom';
import {
  Container,
  Typography,
  Button,
  Box,
  Paper,
  Grid,
  Breadcrumbs,
  Link,
  CircularProgress,
  Alert,
} from '@mui/material';
import AddShoppingCartIcon from '@mui/icons-material/AddShoppingCart';
import ArrowBackIcon from '@mui/icons-material/ArrowBack';
import { productAPI, cartAPI } from '../services/api';

function ProductDetail() {
  const { id } = useParams();
  const navigate = useNavigate();
  const [product, setProduct] = useState(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const [addingToCart, setAddingToCart] = useState(false);

  useEffect(() => {
    loadProduct();
  }, [id]);

  const loadProduct = async () => {
    try {
      setLoading(true);
      setError(null);
      const response = await productAPI.getById(id);
      setProduct(response.data);
    } catch (err) {
      console.error('Failed to load product:', err);
      setError(err.response?.data?.message || 'Failed to load product. Please try again later.');
    } finally {
      setLoading(false);
    }
  };

  const handleAddToCart = async () => {
    try {
      setAddingToCart(true);
      await cartAPI.addItem(product.id, 1);
      // Success - could show a snackbar notification
      alert('Product added to cart!');
    } catch (err) {
      console.error('Failed to add to cart:', err);
      alert(err.response?.data?.message || 'Failed to add to cart. Please try again.');
    } finally {
      setAddingToCart(false);
    }
  };

  if (loading) {
    return (
      <Container maxWidth="lg" sx={{ mt: 4, mb: 4, display: 'flex', justifyContent: 'center' }}>
        <CircularProgress />
      </Container>
    );
  }

  if (error || !product) {
    return (
      <Container maxWidth="lg" sx={{ mt: 4, mb: 4 }}>
        <Alert
          severity="error"
          action={
            <Button color="inherit" size="small" onClick={loadProduct}>
              Retry
            </Button>
          }
        >
          {error || 'Product not found'}
        </Alert>
        <Button
          startIcon={<ArrowBackIcon />}
          component={RouterLink}
          to="/products"
          sx={{ mt: 2 }}
        >
          Back to Products
        </Button>
      </Container>
    );
  }

  return (
    <Container maxWidth="lg" sx={{ mt: 4, mb: 4 }}>
      <Breadcrumbs aria-label="breadcrumb" sx={{ mb: 2 }}>
        <Link component={RouterLink} to="/" underline="hover" color="inherit">
          Home
        </Link>
        <Link component={RouterLink} to="/products" underline="hover" color="inherit">
          Products
        </Link>
        <Typography color="text.primary">{product.name}</Typography>
      </Breadcrumbs>

      <Button
        startIcon={<ArrowBackIcon />}
        component={RouterLink}
        to="/products"
        sx={{ mb: 2 }}
      >
        Back to Products
      </Button>

      <Paper elevation={3} sx={{ p: 3 }}>
        <Grid container spacing={4}>
          <Grid item xs={12} md={6}>
            <Box
              component="img"
              src={product.image_url || `https://via.placeholder.com/600x400?text=${encodeURIComponent(product.name)}`}
              alt={product.name}
              sx={{
                width: '100%',
                height: 'auto',
                borderRadius: 1,
              }}
            />
          </Grid>
          <Grid item xs={12} md={6}>
            <Typography variant="h3" component="h1" gutterBottom>
              {product.name}
            </Typography>
            <Typography variant="h4" color="primary" gutterBottom>
              ${Number(product.price).toFixed(2)}
            </Typography>
            {product.description && (
              <Typography variant="body1" paragraph sx={{ mt: 2 }}>
                {product.description}
              </Typography>
            )}
            <Typography variant="body2" color="text.secondary" paragraph>
              In stock: {product.inventory_count || 0} units
            </Typography>
            <Box sx={{ mt: 3, display: 'flex', gap: 2 }}>
              <Button
                variant="contained"
                size="large"
                startIcon={<AddShoppingCartIcon />}
                onClick={handleAddToCart}
                disabled={addingToCart || !product.inventory_count || product.inventory_count === 0}
              >
                {addingToCart ? 'Adding...' : 'Add to Cart'}
              </Button>
            </Box>
          </Grid>
        </Grid>
      </Paper>
    </Container>
  );
}

export default ProductDetail;

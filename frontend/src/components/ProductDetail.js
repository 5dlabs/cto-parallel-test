import React from 'react';
import { useParams, Link as RouterLink } from 'react-router-dom';
import {
  Container,
  Typography,
  Button,
  Box,
  Paper,
  Grid,
  Breadcrumbs,
  Link,
} from '@mui/material';
import AddShoppingCartIcon from '@mui/icons-material/AddShoppingCart';
import ArrowBackIcon from '@mui/icons-material/ArrowBack';

function ProductDetail() {
  const { id } = useParams();

  // Mock product data
  const product = {
    id,
    name: `Product ${id}`,
    price: 29.99 + id * 10,
    description: 'This is a detailed description of the product. It features high-quality materials and excellent craftsmanship.',
    image: `https://via.placeholder.com/600x400?text=Product+${id}`,
    stock: 15,
  };

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
              src={product.image}
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
              ${product.price}
            </Typography>
            <Typography variant="body1" paragraph sx={{ mt: 2 }}>
              {product.description}
            </Typography>
            <Typography variant="body2" color="text.secondary" paragraph>
              In stock: {product.stock} units
            </Typography>
            <Box sx={{ mt: 3, display: 'flex', gap: 2 }}>
              <Button
                variant="contained"
                size="large"
                startIcon={<AddShoppingCartIcon />}
              >
                Add to Cart
              </Button>
            </Box>
          </Grid>
        </Grid>
      </Paper>
    </Container>
  );
}

export default ProductDetail;

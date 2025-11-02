import React, { useState, useEffect } from 'react';
import { useParams, Link as RouterLink, useNavigate } from 'react-router-dom';
import {
  Container,
  Grid,
  Card,
  CardMedia,
  Typography,
  Button,
  Box,
  TextField,
  CircularProgress,
  Chip,
} from '@mui/material';
import ShoppingCartIcon from '@mui/icons-material/ShoppingCart';
import ArrowBackIcon from '@mui/icons-material/ArrowBack';

// Mock products - same as ProductList
const mockProducts = [
  {
    id: 1,
    name: 'Wireless Headphones',
    description: 'Premium noise-canceling wireless headphones with 30-hour battery life. Features advanced audio technology, comfortable ear cushions, and foldable design for easy portability.',
    price: 299.99,
    image: 'https://via.placeholder.com/500x400/1976d2/ffffff?text=Headphones',
    inventory_count: 50,
    category: 'Audio',
  },
  {
    id: 2,
    name: 'Smart Watch',
    description: 'Fitness tracking smartwatch with heart rate monitor and GPS. Track your workouts, receive notifications, and monitor your health 24/7.',
    price: 399.99,
    image: 'https://via.placeholder.com/500x400/dc004e/ffffff?text=Smart+Watch',
    inventory_count: 30,
    category: 'Wearables',
  },
  {
    id: 3,
    name: 'Laptop Backpack',
    description: 'Durable laptop backpack with multiple compartments and USB charging port. Water-resistant material and ergonomic design for maximum comfort.',
    price: 79.99,
    image: 'https://via.placeholder.com/500x400/388e3c/ffffff?text=Backpack',
    inventory_count: 100,
    category: 'Accessories',
  },
  {
    id: 4,
    name: 'Mechanical Keyboard',
    description: 'RGB mechanical gaming keyboard with Cherry MX switches. Customizable backlighting, programmable keys, and durable construction.',
    price: 149.99,
    image: 'https://via.placeholder.com/500x400/f57c00/ffffff?text=Keyboard',
    inventory_count: 45,
    category: 'Peripherals',
  },
  {
    id: 5,
    name: 'Wireless Mouse',
    description: 'Ergonomic wireless mouse with precision tracking. Long battery life and comfortable design for all-day use.',
    price: 59.99,
    image: 'https://via.placeholder.com/500x400/7b1fa2/ffffff?text=Mouse',
    inventory_count: 75,
    category: 'Peripherals',
  },
  {
    id: 6,
    name: '4K Webcam',
    description: 'Ultra HD 4K webcam with auto-focus and noise reduction. Perfect for video calls, streaming, and content creation.',
    price: 199.99,
    image: 'https://via.placeholder.com/500x400/0288d1/ffffff?text=Webcam',
    inventory_count: 25,
    category: 'Peripherals',
  },
  {
    id: 7,
    name: 'USB-C Hub',
    description: '7-in-1 USB-C hub with HDMI, USB 3.0, and SD card reader. Expand your laptop connectivity with this compact hub.',
    price: 49.99,
    image: 'https://via.placeholder.com/500x400/303f9f/ffffff?text=USB+Hub',
    inventory_count: 60,
    category: 'Accessories',
  },
  {
    id: 8,
    name: 'Portable SSD',
    description: '1TB portable SSD with USB-C interface and 540MB/s transfer speed. Fast, reliable storage for your files.',
    price: 129.99,
    image: 'https://via.placeholder.com/500x400/c2185b/ffffff?text=SSD',
    inventory_count: 40,
    category: 'Storage',
  },
];

function ProductDetail() {
  const { id } = useParams();
  const navigate = useNavigate();
  const [product, setProduct] = useState(null);
  const [loading, setLoading] = useState(true);
  const [quantity, setQuantity] = useState(1);

  useEffect(() => {
    // Simulate API call
    const fetchProduct = async () => {
      setLoading(true);
      // In real app: const response = await axios.get(`/api/products/${id}`);
      await new Promise((resolve) => setTimeout(resolve, 500));
      const foundProduct = mockProducts.find((p) => p.id === parseInt(id));
      setProduct(foundProduct);
      setLoading(false);
    };

    fetchProduct();
  }, [id]);

  const handleAddToCart = () => {
    // In real app, this would add to cart state/context
    console.log('Add to cart:', product.id, 'quantity:', quantity);
    alert(`Added ${quantity} ${product.name}(s) to cart!`);
  };

  const handleQuantityChange = (event) => {
    const value = parseInt(event.target.value);
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
      <Container maxWidth="lg" sx={{ py: 8 }}>
        <Typography variant="h4">Product not found</Typography>
        <Button
          component={RouterLink}
          to="/products"
          startIcon={<ArrowBackIcon />}
          sx={{ mt: 2 }}
        >
          Back to Products
        </Button>
      </Container>
    );
  }

  return (
    <Container maxWidth="lg" sx={{ py: 8 }}>
      <Button
        component={RouterLink}
        to="/products"
        startIcon={<ArrowBackIcon />}
        sx={{ mb: 4 }}
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
          <Box>
            <Typography component="h1" variant="h3" gutterBottom sx={{ fontWeight: 'bold' }}>
              {product.name}
            </Typography>
            <Chip label={product.category} color="primary" sx={{ mb: 2 }} />
            <Typography variant="h4" color="primary" gutterBottom sx={{ fontWeight: 'bold', my: 2 }}>
              ${product.price.toFixed(2)}
            </Typography>
            <Typography variant="body1" color="text.secondary" paragraph sx={{ fontSize: '1.1rem' }}>
              {product.description}
            </Typography>
            <Box sx={{ my: 3 }}>
              <Typography variant="body2" color="text.secondary" gutterBottom>
                Availability:{' '}
                <Typography
                  component="span"
                  variant="body2"
                  color={product.inventory_count > 0 ? 'success.main' : 'error.main'}
                  sx={{ fontWeight: 'bold' }}
                >
                  {product.inventory_count > 0
                    ? `${product.inventory_count} in stock`
                    : 'Out of stock'}
                </Typography>
              </Typography>
            </Box>
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
                sx={{ width: 100 }}
                disabled={product.inventory_count === 0}
              />
              <Button
                variant="contained"
                size="large"
                startIcon={<ShoppingCartIcon />}
                onClick={handleAddToCart}
                disabled={product.inventory_count === 0}
                sx={{ flexGrow: 1 }}
              >
                Add to Cart
              </Button>
            </Box>
          </Box>
        </Grid>
      </Grid>
    </Container>
  );
}

export default ProductDetail;

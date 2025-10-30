import { Container, Grid, Card, CardContent, CardMedia, CardActions, Typography, Button, Box } from '@mui/material';
import { Link } from 'react-router-dom';

function ProductList() {
  // TODO: Replace with API call to backend
  const products = [
    {
      id: 1,
      name: 'Product 1',
      price: 19.99,
      description: 'This is a great product with excellent features and quality craftsmanship.',
    },
    {
      id: 2,
      name: 'Product 2',
      price: 29.99,
      description: 'An amazing product that will exceed your expectations in every way.',
    },
    {
      id: 3,
      name: 'Product 3',
      price: 39.99,
      description: 'Premium quality product designed for those who demand the best.',
    },
  ];

  const handleAddToCart = (productId) => {
    // TODO: Connect to cart API
    console.log(`Adding product ${productId} to cart`);
  };

  return (
    <Container maxWidth="lg">
      <Typography variant="h3" component="h1" gutterBottom sx={{ mb: 4 }}>
        Our Products
      </Typography>
      <Grid container spacing={4}>
        {products.map((product) => (
          <Grid item key={product.id} xs={12} sm={6} md={4}>
            <Card sx={{ height: '100%', display: 'flex', flexDirection: 'column' }}>
              <CardMedia
                sx={{
                  height: 200,
                  backgroundColor: 'grey.300',
                }}
                title={product.name}
              >
                <Box
                  sx={{
                    height: '100%',
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'center',
                    color: 'grey.600',
                  }}
                >
                  <Typography variant="h6">Image Placeholder</Typography>
                </Box>
              </CardMedia>
              <CardContent sx={{ flexGrow: 1 }}>
                <Typography gutterBottom variant="h5" component="h2">
                  {product.name}
                </Typography>
                <Typography variant="h6" color="primary" gutterBottom>
                  ${product.price.toFixed(2)}
                </Typography>
                <Typography variant="body2" color="text.secondary">
                  {product.description}
                </Typography>
              </CardContent>
              <CardActions sx={{ p: 2, pt: 0 }}>
                <Button
                  size="small"
                  variant="outlined"
                  component={Link}
                  to={`/products/${product.id}`}
                  fullWidth
                >
                  View Details
                </Button>
                <Button
                  size="small"
                  variant="contained"
                  color="primary"
                  onClick={() => handleAddToCart(product.id)}
                  fullWidth
                >
                  Add to Cart
                </Button>
              </CardActions>
            </Card>
          </Grid>
        ))}
      </Grid>
    </Container>
  );
}

export default ProductList;

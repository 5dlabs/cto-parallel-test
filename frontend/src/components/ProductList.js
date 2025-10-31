import React from 'react';
import { useNavigate } from 'react-router-dom';
import Container from '@mui/material/Container';
import Grid from '@mui/material/Grid';
import Card from '@mui/material/Card';
import CardContent from '@mui/material/CardContent';
import CardMedia from '@mui/material/CardMedia';
import CardActions from '@mui/material/CardActions';
import Typography from '@mui/material/Typography';
import Button from '@mui/material/Button';
import Box from '@mui/material/Box';

function ProductList() {
  const navigate = useNavigate();

  // TODO: Replace with API call to backend
  const products = [
    {
      id: 1,
      name: 'Product 1',
      price: 19.99,
      description: 'This is a great product with excellent features and quality.',
    },
    {
      id: 2,
      name: 'Product 2',
      price: 29.99,
      description: 'An amazing product that will meet all your needs.',
    },
    {
      id: 3,
      name: 'Product 3',
      price: 39.99,
      description: 'Premium quality product with advanced capabilities.',
    },
  ];

  const handleViewDetails = (productId) => {
    navigate(`/products/${productId}`);
  };

  const handleAddToCart = (productId) => {
    // TODO: Connect to cart API
    console.log(`Adding product ${productId} to cart`);
  };

  return (
    <Container maxWidth="lg">
      <Box sx={{ my: 4 }}>
        <Typography variant="h3" component="h1" gutterBottom align="center">
          Our Products
        </Typography>
        <Grid container spacing={4} sx={{ mt: 2 }}>
          {products.map((product) => (
            <Grid item key={product.id} xs={12} sm={6} md={4}>
              <Card
                sx={{
                  height: '100%',
                  display: 'flex',
                  flexDirection: 'column',
                }}
              >
                <CardMedia
                  sx={{
                    height: 200,
                    backgroundColor: 'grey.300',
                  }}
                  title={product.name}
                />
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
                <CardActions sx={{ justifyContent: 'space-between', px: 2, pb: 2 }}>
                  <Button
                    size="small"
                    variant="outlined"
                    onClick={() => handleViewDetails(product.id)}
                  >
                    View Details
                  </Button>
                  <Button
                    size="small"
                    variant="contained"
                    color="primary"
                    onClick={() => handleAddToCart(product.id)}
                  >
                    Add to Cart
                  </Button>
                </CardActions>
              </Card>
            </Grid>
          ))}
        </Grid>
      </Box>
    </Container>
  );
}

export default ProductList;

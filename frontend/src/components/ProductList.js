import React from 'react';
import { Link } from 'react-router-dom';
import { 
  Container, 
  Grid, 
  Card, 
  CardContent, 
  CardMedia, 
  CardActions, 
  Typography, 
  Button 
} from '@mui/material';

function ProductList() {
  // Hardcoded placeholder products
  const products = [
    {
      id: 1,
      name: 'Wireless Headphones',
      price: 79.99,
      description: 'High-quality wireless headphones with noise cancellation',
      image: 'https://via.placeholder.com/300x200?text=Headphones'
    },
    {
      id: 2,
      name: 'Smart Watch',
      price: 199.99,
      description: 'Feature-rich smartwatch with fitness tracking',
      image: 'https://via.placeholder.com/300x200?text=Smart+Watch'
    },
    {
      id: 3,
      name: 'Laptop Stand',
      price: 49.99,
      description: 'Ergonomic aluminum laptop stand',
      image: 'https://via.placeholder.com/300x200?text=Laptop+Stand'
    }
  ];

  return (
    <Container sx={{ py: 4 }}>
      <Typography variant="h3" component="h1" gutterBottom align="center">
        Our Products
      </Typography>
      
      <Grid container spacing={4} sx={{ mt: 2 }}>
        {products.map((product) => (
          <Grid item key={product.id} xs={12} sm={6} md={4}>
            <Card sx={{ height: '100%', display: 'flex', flexDirection: 'column' }}>
              <CardMedia
                component="img"
                height="200"
                image={product.image}
                alt={product.name}
              />
              <CardContent sx={{ flexGrow: 1 }}>
                <Typography gutterBottom variant="h5" component="h2">
                  {product.name}
                </Typography>
                <Typography variant="h6" color="primary" gutterBottom>
                  ${product.price}
                </Typography>
                <Typography variant="body2" color="text.secondary">
                  {product.description}
                </Typography>
              </CardContent>
              <CardActions>
                <Button 
                  size="small" 
                  component={Link} 
                  to={`/products/${product.id}`}
                >
                  View Details
                </Button>
                <Button size="small" variant="contained">
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

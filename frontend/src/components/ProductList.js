import React from 'react';
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
} from '@mui/material';
import AddShoppingCartIcon from '@mui/icons-material/AddShoppingCart';

const mockProducts = [
  { id: 1, name: 'Product 1', price: 29.99, image: 'https://via.placeholder.com/300x200?text=Product+1' },
  { id: 2, name: 'Product 2', price: 39.99, image: 'https://via.placeholder.com/300x200?text=Product+2' },
  { id: 3, name: 'Product 3', price: 49.99, image: 'https://via.placeholder.com/300x200?text=Product+3' },
  { id: 4, name: 'Product 4', price: 59.99, image: 'https://via.placeholder.com/300x200?text=Product+4' },
  { id: 5, name: 'Product 5', price: 69.99, image: 'https://via.placeholder.com/300x200?text=Product+5' },
  { id: 6, name: 'Product 6', price: 79.99, image: 'https://via.placeholder.com/300x200?text=Product+6' },
];

function ProductList() {
  return (
    <Container maxWidth="lg" sx={{ mt: 4, mb: 4 }}>
      <Typography variant="h3" component="h1" gutterBottom>
        Products
      </Typography>
      <Grid container spacing={3}>
        {mockProducts.map((product) => (
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
                <Typography variant="h6" color="primary">
                  ${product.price}
                </Typography>
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

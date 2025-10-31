import React from 'react';
import { Link as RouterLink } from 'react-router-dom';
import {
  Container,
  Grid,
  Card,
  CardMedia,
  CardContent,
  CardActions,
  Typography,
  Button,
  Stack,
} from '@mui/material';

const products = [
  {
    id: 1,
    name: 'Wireless Headphones',
    price: '$199.99',
    description: 'Experience superior sound quality with noise cancellation and long battery life.',
    image: 'https://via.placeholder.com/400x250?text=Headphones',
  },
  {
    id: 2,
    name: 'Smart Watch',
    price: '$149.99',
    description: 'Track your fitness, stay connected, and enjoy customizable watch faces.',
    image: 'https://via.placeholder.com/400x250?text=Smart+Watch',
  },
  {
    id: 3,
    name: '4K Monitor',
    price: '$399.99',
    description: 'Vibrant colors and ultra-high definition display for work and entertainment.',
    image: 'https://via.placeholder.com/400x250?text=4K+Monitor',
  },
];

const ProductList = () => (
  <Container maxWidth="lg">
    <Stack spacing={4}>
      <Typography variant="h4" component="h1" sx={{ fontWeight: 600 }}>
        Featured Products
      </Typography>
      <Grid container spacing={4}>
        {products.map((product) => (
          <Grid item xs={12} sm={6} md={4} key={product.id}>
            <Card elevation={4} sx={{ height: '100%', display: 'flex', flexDirection: 'column' }}>
              <CardMedia component="img" height="200" image={product.image} alt={product.name} />
              <CardContent sx={{ flexGrow: 1 }}>
                <Typography gutterBottom variant="h6" component="div">
                  {product.name}
                </Typography>
                <Typography variant="subtitle1" color="secondary" sx={{ fontWeight: 600 }}>
                  {product.price}
                </Typography>
                <Typography variant="body2" color="text.secondary" sx={{ mt: 1 }}>
                  {product.description}
                </Typography>
              </CardContent>
              <CardActions sx={{ justifyContent: 'space-between', px: 2, pb: 2 }}>
                <Button
                  size="small"
                  variant="outlined"
                  component={RouterLink}
                  to={`/products/${product.id}`}
                >
                  View Details
                </Button>
                <Button size="small" variant="contained" color="secondary">
                  Add to Cart
                </Button>
              </CardActions>
            </Card>
          </Grid>
        ))}
      </Grid>
    </Stack>
  </Container>
);

export default ProductList;

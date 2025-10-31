import React from 'react';
import {
  Grid,
  Card,
  CardMedia,
  CardContent,
  Typography,
  CardActions,
  Button,
  Chip,
  Stack,
} from '@mui/material';
import { Link as RouterLink } from 'react-router-dom';

const products = [
  {
    id: 1,
    name: 'Wireless Headphones',
    price: '$129.99',
    description: 'Experience immersive sound with active noise cancellation and 30-hour battery life.',
  },
  {
    id: 2,
    name: 'Smart Watch',
    price: '$249.99',
    description: 'Track your health and stay connected with GPS, heart-rate monitoring, and mobile payments.',
  },
  {
    id: 3,
    name: '4K Ultra HD TV',
    price: '$799.99',
    description: 'Stunning visuals with HDR10+ support and built-in streaming apps for endless entertainment.',
  },
];

const ProductList = () => (
  <Grid container spacing={3}>
    {products.map((product) => (
      <Grid item xs={12} sm={6} md={4} key={product.id}>
        <Card sx={{ height: '100%', display: 'flex', flexDirection: 'column' }}>
          <CardMedia
            component="img"
            height="200"
            image={`https://via.placeholder.com/400x300.png?text=${encodeURIComponent(product.name)}`}
            alt={`${product.name} product visual`}
          />
          <CardContent sx={{ flexGrow: 1 }}>
            <Stack spacing={1}>
              <Typography gutterBottom variant="h6" component="h2">
                {product.name}
              </Typography>
              <Chip
                label={product.price}
                color="primary"
                sx={{ width: 'fit-content', fontWeight: 600 }}
              />
              <Typography variant="body2" color="text.secondary">
                {product.description}
              </Typography>
            </Stack>
          </CardContent>
          <CardActions sx={{ justifyContent: 'space-between', px: 2, pb: 2 }}>
            <Button
              size="small"
              component={RouterLink}
              to={`/products/${product.id}`}
              variant="outlined"
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
);

export default ProductList;

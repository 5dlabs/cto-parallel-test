import React from 'react';
import { Link as RouterLink } from 'react-router-dom';
import Grid from '@mui/material/Grid';
import Card from '@mui/material/Card';
import CardContent from '@mui/material/CardContent';
import CardActions from '@mui/material/CardActions';
import CardMedia from '@mui/material/CardMedia';
import Typography from '@mui/material/Typography';
import Button from '@mui/material/Button';
import Box from '@mui/material/Box';

const products = [
  {
    id: 1,
    name: 'Wireless Headphones',
    price: '$199.99',
    description: 'Experience immersive sound with noise cancellation and 24-hour battery life.',
    image: 'https://via.placeholder.com/300x200?text=Headphones',
  },
  {
    id: 2,
    name: 'Smart Watch',
    price: '$249.99',
    description: 'Track fitness, receive notifications, and stay connected on the go.',
    image: 'https://via.placeholder.com/300x200?text=Smart+Watch',
  },
  {
    id: 3,
    name: 'Portable Speaker',
    price: '$129.99',
    description: 'Compact design with powerful sound and waterproof durability.',
    image: 'https://via.placeholder.com/300x200?text=Speaker',
  },
];

function ProductList() {
  return (
    <Box sx={{ flexGrow: 1 }}>
      <Typography variant="h4" component="h2" gutterBottom fontWeight={600}>
        Featured Products
      </Typography>
      <Grid container spacing={4}>
        {products.map((product) => (
          <Grid item key={product.id} xs={12} sm={6} md={4}>
            <Card sx={{ height: '100%', display: 'flex', flexDirection: 'column' }}>
              <CardMedia component="img" height="200" image={product.image} alt={product.name} />
              <CardContent sx={{ flexGrow: 1 }}>
                <Typography gutterBottom variant="h6" component="div">
                  {product.name}
                </Typography>
                <Typography variant="subtitle1" color="primary" fontWeight={600}>
                  {product.price}
                </Typography>
                <Typography variant="body2" color="text.secondary" mt={1}>
                  {product.description}
                </Typography>
              </CardContent>
              <CardActions sx={{ justifyContent: 'space-between' }}>
                <Button component={RouterLink} to={`/products/${product.id}`} size="small">
                  View Details
                </Button>
                <Button variant="contained" color="primary" size="small">
                  Add to Cart
                </Button>
              </CardActions>
            </Card>
          </Grid>
        ))}
      </Grid>
    </Box>
  );
}

export default ProductList;

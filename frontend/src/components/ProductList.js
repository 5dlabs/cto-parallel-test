import Container from '@mui/material/Container';
import Grid from '@mui/material/Grid';
import Card from '@mui/material/Card';
import CardContent from '@mui/material/CardContent';
import CardMedia from '@mui/material/CardMedia';
import Typography from '@mui/material/Typography';
import Button from '@mui/material/Button';
import CardActions from '@mui/material/CardActions';
import Stack from '@mui/material/Stack';
import { Link as RouterLink } from 'react-router-dom';

const products = [
  {
    id: 1,
    name: 'Wireless Headphones',
    price: 199.99,
    description: 'Experience immersive sound with active noise cancellation and long battery life.',
  },
  {
    id: 2,
    name: 'Smart Fitness Watch',
    price: 149.99,
    description: 'Track your health metrics, workouts, and sleep patterns with advanced sensors.',
  },
  {
    id: 3,
    name: '4K Ultra HD Monitor',
    price: 329.99,
    description: 'Enjoy crystal clear visuals with HDR support and ultra-thin bezels.',
  },
];

const ProductList = () => (
  <Container maxWidth="lg">
    <Stack spacing={4} sx={{ mt: 4 }}>
      <Typography variant="h4" component="h2" fontWeight={600}>
        Featured Products
      </Typography>
      <Grid container spacing={4}>
        {products.map((product) => (
          <Grid item key={product.id} xs={12} sm={6} md={4}>
            <Card sx={{ height: '100%', display: 'flex', flexDirection: 'column' }}>
              <CardMedia
                component="div"
                sx={{
                  pt: '56.25%',
                  backgroundColor: 'grey.200',
                  display: 'flex',
                  alignItems: 'center',
                  justifyContent: 'center',
                  color: 'text.secondary',
                  fontSize: 18,
                  fontWeight: 500,
                }}
              >
                Image Coming Soon
              </CardMedia>
              <CardContent sx={{ flexGrow: 1 }}>
                <Typography gutterBottom variant="h6" component="div">
                  {product.name}
                </Typography>
                <Typography variant="subtitle1" color="primary" fontWeight={700}>
                  ${product.price.toFixed(2)}
                </Typography>
                <Typography variant="body2" color="text.secondary" sx={{ mt: 1.5 }}>
                  {product.description}
                </Typography>
              </CardContent>
              <CardActions sx={{ justifyContent: 'space-between', px: 2, pb: 2 }}>
                <Button
                  size="small"
                  component={RouterLink}
                  to={`/products/${product.id}`}
                  variant="outlined"
                  color="primary"
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

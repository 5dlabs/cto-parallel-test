import React from 'react';
import { useParams, Link as RouterLink } from 'react-router-dom';
import {
  Container,
  Typography,
  Button,
  Box,
  Paper,
} from '@mui/material';
import ArrowBackIcon from '@mui/icons-material/ArrowBack';

function ProductDetail() {
  const { id } = useParams();

  // Placeholder - will be replaced with API call to fetch product details
  return (
    <Container maxWidth="md">
      <Box sx={{ mb: 3 }}>
        <Button
          component={RouterLink}
          to="/products"
          startIcon={<ArrowBackIcon />}
        >
          Back to Products
        </Button>
      </Box>

      <Paper elevation={3} sx={{ p: 4 }}>
        <Typography variant="h4" component="h1" gutterBottom>
          Product Detail - ID: {id}
        </Typography>

        <Box
          sx={{
            width: '100%',
            height: 300,
            backgroundColor: 'grey.300',
            mb: 3,
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
          }}
        >
          <Typography variant="h6" color="text.secondary">
            Product Image Placeholder
          </Typography>
        </Box>

        <Typography variant="body1" color="text.secondary" paragraph>
          This is a placeholder for product details. In the future, this page
          will display comprehensive information about the product including:
        </Typography>

        <Box component="ul" sx={{ pl: 3 }}>
          <Typography component="li" variant="body1">
            Product name and description
          </Typography>
          <Typography component="li" variant="body1">
            Price and availability
          </Typography>
          <Typography component="li" variant="body1">
            Product specifications
          </Typography>
          <Typography component="li" variant="body1">
            Customer reviews
          </Typography>
        </Box>

        <Box sx={{ mt: 4 }}>
          <Button variant="contained" color="primary" size="large">
            Add to Cart
          </Button>
        </Box>
      </Paper>
    </Container>
  );
}

export default ProductDetail;

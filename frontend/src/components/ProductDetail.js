import React from 'react';
import { useParams, Link } from 'react-router-dom';
import { Container, Typography, Button, Box } from '@mui/material';
import ArrowBackIcon from '@mui/icons-material/ArrowBack';

function ProductDetail() {
  const { id } = useParams();

  return (
    <Container sx={{ py: 4 }}>
      <Button 
        component={Link} 
        to="/products" 
        startIcon={<ArrowBackIcon />}
        sx={{ mb: 3 }}
      >
        Back to Products
      </Button>
      
      <Box sx={{ mt: 4, textAlign: 'center' }}>
        <Typography variant="h3" component="h1" gutterBottom>
          Product Detail - ID: {id}
        </Typography>
        <Typography variant="body1" color="text.secondary" sx={{ mt: 2 }}>
          Detailed product information will be displayed here.
        </Typography>
        <Typography variant="body2" color="text.secondary" sx={{ mt: 1 }}>
          This is a placeholder for future product details integration.
        </Typography>
      </Box>
    </Container>
  );
}

export default ProductDetail;

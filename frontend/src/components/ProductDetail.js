import React from 'react';
import { useParams, Link } from 'react-router-dom';
import Container from '@mui/material/Container';
import Typography from '@mui/material/Typography';
import Button from '@mui/material/Button';
import Box from '@mui/material/Box';
import Paper from '@mui/material/Paper';

function ProductDetail() {
  const { id } = useParams();

  return (
    <Container maxWidth="md">
      <Paper elevation={3} sx={{ p: 4, mt: 2 }}>
        <Typography variant="h3" component="h1" gutterBottom>
          Product Detail - ID: {id}
        </Typography>
        <Typography variant="body1" color="text.secondary" paragraph>
          This is a placeholder page for product details. 
          In the future, this will display full product information 
          fetched from the API.
        </Typography>
        <Box sx={{ mt: 3 }}>
          <Button
            variant="outlined"
            component={Link}
            to="/products"
            sx={{ mr: 2 }}
          >
            Back to Products
          </Button>
          <Button variant="contained" color="primary">
            Add to Cart
          </Button>
        </Box>
      </Paper>
    </Container>
  );
}

export default ProductDetail;

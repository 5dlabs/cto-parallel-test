import React from 'react';
import { useParams } from 'react-router-dom';
import { Container, Typography, Box } from '@mui/material';

function ProductDetail() {
  const { id } = useParams();

  return (
    <Container maxWidth="md">
      <Box sx={{ mt: 4 }}>
        <Typography variant="h3" component="h1" gutterBottom>
          Product Detail - ID: {id}
        </Typography>

        <Typography variant="body1" color="text.secondary" paragraph>
          Detailed product information will be displayed here.
        </Typography>

        <Typography variant="body2" color="text.secondary">
          This is a placeholder component. Future implementation will fetch
          product details from the API.
        </Typography>
      </Box>
    </Container>
  );
}

export default ProductDetail;

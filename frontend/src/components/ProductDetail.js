import React from 'react';
import { useParams } from 'react-router-dom';
import { Container, Typography, Box, Paper } from '@mui/material';

function ProductDetail() {
  const { id } = useParams();

  return (
    <Container maxWidth="md">
      <Paper elevation={3} sx={{ p: 4, mt: 4 }}>
        <Typography variant="h3" component="h1" gutterBottom>
          Product Detail - ID: {id}
        </Typography>
        <Box sx={{ mt: 3 }}>
          <Typography variant="body1" color="text.secondary">
            This is a placeholder for product details. In a future implementation,
            this page will fetch and display full product information from the API
            including images, detailed description, specifications, and reviews.
          </Typography>
        </Box>
      </Paper>
    </Container>
  );
}

export default ProductDetail;

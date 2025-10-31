import React from 'react';
import { useParams, Link as RouterLink } from 'react-router-dom';
import { Container, Typography, Button, Stack } from '@mui/material';

const ProductDetail = () => {
  const { id } = useParams();

  return (
    <Container maxWidth="md">
      <Stack spacing={3}>
        <Typography variant="h4" component="h1" sx={{ fontWeight: 600 }}>
          Product Detail - ID: {id}
        </Typography>
        <Typography variant="body1" color="text.secondary">
          Product information will appear here once connected to the backend.
        </Typography>
        <Button variant="contained" component={RouterLink} to="/products">
          Back to Products
        </Button>
      </Stack>
    </Container>
  );
};

export default ProductDetail;

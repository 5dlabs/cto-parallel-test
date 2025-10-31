import React from 'react';
import { useParams } from 'react-router-dom';
import { Paper, Typography, Stack, Button } from '@mui/material';
import { Link as RouterLink } from 'react-router-dom';

const ProductDetail = () => {
  const { id } = useParams();

  return (
    <Paper sx={{ p: { xs: 3, md: 4 } }} elevation={2}>
      <Stack spacing={2}>
        <Typography variant="h4" component="h1">
          Product Detail - ID: {id}
        </Typography>
        <Typography variant="body1" color="text.secondary">
          Detailed product information, specifications, and reviews will appear here once this page is
          connected to live data sources.
        </Typography>
        <Button
          variant="outlined"
          component={RouterLink}
          to="/products"
          sx={{ alignSelf: { xs: 'stretch', sm: 'flex-start' } }}
        >
          Back to Products
        </Button>
      </Stack>
    </Paper>
  );
};

export default ProductDetail;

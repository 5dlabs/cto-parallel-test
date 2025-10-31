import React from 'react';
import { Box, Stack, Typography, Button } from '@mui/material';
import { Link as RouterLink } from 'react-router-dom';

const HomePage = () => (
  <Box textAlign="center" py={8}>
    <Stack spacing={3} alignItems="center">
      <Typography variant="h3" component="h1" fontWeight={700} gutterBottom>
        Welcome to ShopSmart
      </Typography>
      <Typography
        variant="subtitle1"
        color="text.secondary"
        maxWidth={600}
        sx={{ px: { xs: 2, sm: 0 } }}
      >
        Discover curated products at unbeatable prices. Shop the latest trends and must-have
        essentials with fast, reliable delivery.
      </Typography>
      <Button
        variant="contained"
        color="primary"
        size="large"
        component={RouterLink}
        to="/products"
      >
        Shop Now
      </Button>
    </Stack>
  </Box>
);

export default HomePage;

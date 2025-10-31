import React from 'react';
import { Link as RouterLink } from 'react-router-dom';
import { Container, Typography, Button, Box } from '@mui/material';

const HomePage = () => (
  <Container maxWidth="lg">
    <Box
      sx={{
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        textAlign: 'center',
        gap: 3,
      }}
    >
      <Typography variant="h3" component="h1" sx={{ fontWeight: 700 }}>
        Welcome to E-Shop
      </Typography>
      <Typography variant="h6" color="text.secondary">
        Discover the latest products curated just for you. Enjoy a seamless shopping experience across devices.
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
    </Box>
  </Container>
);

export default HomePage;

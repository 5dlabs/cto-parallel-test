import React from 'react';
import { Link as RouterLink } from 'react-router-dom';
import Box from '@mui/material/Box';
import Typography from '@mui/material/Typography';
import Button from '@mui/material/Button';

function HomePage() {
  return (
    <Box
      sx={{
        textAlign: 'center',
        py: { xs: 6, md: 10 },
        display: 'flex',
        flexDirection: 'column',
        gap: 3,
        alignItems: 'center',
      }}
    >
      <Typography variant="h3" component="h1" fontWeight={700} gutterBottom>
        Welcome to ShopSmart
      </Typography>
      <Typography variant="h6" color="text.secondary" maxWidth={600}>
        Discover a curated selection of products designed to elevate your everyday life. Shop our latest arrivals and exclusive deals today.
      </Typography>
      <Button
        component={RouterLink}
        to="/products"
        variant="contained"
        color="secondary"
        size="large"
      >
        Shop Now
      </Button>
    </Box>
  );
}

export default HomePage;

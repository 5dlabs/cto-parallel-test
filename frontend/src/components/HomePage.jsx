import React from 'react';
import { Link as RouterLink } from 'react-router-dom';
import { Container, Typography, Button, Box } from '@mui/material';

function HomePage() {
  return (
    <Container maxWidth="md">
      <Box
        sx={{
          display: 'flex',
          flexDirection: 'column',
          alignItems: 'center',
          justifyContent: 'center',
          minHeight: '60vh',
          textAlign: 'center',
        }}
      >
        <Typography variant="h2" component="h1" gutterBottom>
          Welcome to E-Commerce App
        </Typography>
        <Typography variant="h5" component="h2" gutterBottom color="text.secondary">
          Discover amazing products at great prices
        </Typography>
        <Button
          variant="contained"
          color="primary"
          size="large"
          component={RouterLink}
          to="/products"
          sx={{ mt: 4 }}
        >
          Shop Now
        </Button>
      </Box>
    </Container>
  );
}

export default HomePage;

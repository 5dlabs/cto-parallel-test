import React from 'react';
import { Link as RouterLink } from 'react-router-dom';
import Container from '@mui/material/Container';
import Typography from '@mui/material/Typography';
import Button from '@mui/material/Button';
import Box from '@mui/material/Box';

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
          Welcome to Our E-Commerce Store
        </Typography>

        <Typography variant="h5" color="text.secondary" paragraph>
          Discover amazing products at great prices
        </Typography>

        <Button
          variant="contained"
          color="primary"
          size="large"
          component={RouterLink}
          to="/products"
          sx={{ mt: 3 }}
        >
          Shop Now
        </Button>
      </Box>
    </Container>
  );
}

export default HomePage;

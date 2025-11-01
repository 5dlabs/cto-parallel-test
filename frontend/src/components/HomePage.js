import React from 'react';
import { Link as RouterLink } from 'react-router-dom';
import {
  Container,
  Typography,
  Button,
  Box,
  Paper,
} from '@mui/material';
import ShoppingBagIcon from '@mui/icons-material/ShoppingBag';

function HomePage() {
  return (
    <Container maxWidth="lg" sx={{ mt: 4, mb: 4 }}>
      <Paper
        elevation={3}
        sx={{
          p: 4,
          display: 'flex',
          flexDirection: 'column',
          alignItems: 'center',
          minHeight: '60vh',
          justifyContent: 'center',
          backgroundImage: 'linear-gradient(120deg, #f6f9fc 0%, #e9f2f9 100%)',
        }}
      >
        <ShoppingBagIcon sx={{ fontSize: 80, color: 'primary.main', mb: 2 }} />
        <Typography variant="h2" component="h1" gutterBottom align="center">
          Welcome to E-commerce
        </Typography>
        <Typography variant="h5" color="text.secondary" paragraph align="center">
          Discover amazing products at great prices
        </Typography>
        <Box sx={{ mt: 4, display: 'flex', gap: 2, flexWrap: 'wrap', justifyContent: 'center' }}>
          <Button
            variant="contained"
            size="large"
            component={RouterLink}
            to="/products"
          >
            Shop Now
          </Button>
          <Button
            variant="outlined"
            size="large"
            component={RouterLink}
            to="/register"
          >
            Sign Up
          </Button>
        </Box>
      </Paper>
    </Container>
  );
}

export default HomePage;

import React from 'react';
import { Link as RouterLink } from 'react-router-dom';
import {
  Container,
  Box,
  Typography,
  Button,
  Grid,
  Card,
  CardContent,
  CardActions,
} from '@mui/material';
import ShoppingCartIcon from '@mui/icons-material/ShoppingCart';
import LocalShippingIcon from '@mui/icons-material/LocalShipping';
import VerifiedUserIcon from '@mui/icons-material/VerifiedUser';

function HomePage() {
  const features = [
    {
      icon: <ShoppingCartIcon sx={{ fontSize: 48, color: 'primary.main' }} />,
      title: 'Wide Selection',
      description: 'Browse thousands of products across multiple categories',
    },
    {
      icon: <LocalShippingIcon sx={{ fontSize: 48, color: 'primary.main' }} />,
      title: 'Fast Shipping',
      description: 'Free shipping on orders over $50 with expedited options available',
    },
    {
      icon: <VerifiedUserIcon sx={{ fontSize: 48, color: 'primary.main' }} />,
      title: 'Secure Shopping',
      description: 'Your payment information is protected with enterprise-grade security',
    },
  ];

  return (
    <Container maxWidth="lg">
      {/* Hero Section */}
      <Box
        sx={{
          py: 8,
          textAlign: 'center',
        }}
      >
        <Typography
          component="h1"
          variant="h2"
          color="text.primary"
          gutterBottom
          sx={{ fontWeight: 'bold' }}
        >
          Welcome to E-Commerce Store
        </Typography>
        <Typography variant="h5" color="text.secondary" paragraph>
          Discover amazing products at unbeatable prices. Shop now and enjoy free shipping on orders over $50!
        </Typography>
        <Box sx={{ mt: 4, display: 'flex', gap: 2, justifyContent: 'center' }}>
          <Button
            component={RouterLink}
            to="/products"
            variant="contained"
            size="large"
            startIcon={<ShoppingCartIcon />}
          >
            Shop Now
          </Button>
          <Button
            component={RouterLink}
            to="/register"
            variant="outlined"
            size="large"
          >
            Sign Up
          </Button>
        </Box>
      </Box>

      {/* Features Section */}
      <Box sx={{ py: 8 }}>
        <Typography
          component="h2"
          variant="h4"
          color="text.primary"
          gutterBottom
          sx={{ textAlign: 'center', mb: 6, fontWeight: 'bold' }}
        >
          Why Shop With Us?
        </Typography>
        <Grid container spacing={4}>
          {features.map((feature, index) => (
            <Grid item xs={12} sm={6} md={4} key={index}>
              <Card
                sx={{
                  height: '100%',
                  display: 'flex',
                  flexDirection: 'column',
                  alignItems: 'center',
                  textAlign: 'center',
                  p: 2,
                }}
              >
                <Box sx={{ mb: 2 }}>{feature.icon}</Box>
                <CardContent>
                  <Typography gutterBottom variant="h5" component="h3">
                    {feature.title}
                  </Typography>
                  <Typography color="text.secondary">
                    {feature.description}
                  </Typography>
                </CardContent>
              </Card>
            </Grid>
          ))}
        </Grid>
      </Box>

      {/* Call to Action */}
      <Box
        sx={{
          py: 6,
          textAlign: 'center',
          backgroundColor: 'primary.main',
          color: 'white',
          borderRadius: 2,
          mb: 8,
        }}
      >
        <Typography variant="h4" component="h2" gutterBottom>
          Ready to Start Shopping?
        </Typography>
        <Typography variant="h6" paragraph>
          Join thousands of satisfied customers today!
        </Typography>
        <Button
          component={RouterLink}
          to="/products"
          variant="contained"
          size="large"
          sx={{
            backgroundColor: 'white',
            color: 'primary.main',
            '&:hover': {
              backgroundColor: 'grey.100',
            },
          }}
        >
          Browse Products
        </Button>
      </Box>
    </Container>
  );
}

export default HomePage;

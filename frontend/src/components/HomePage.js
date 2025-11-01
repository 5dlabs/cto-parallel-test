import React from 'react';
import { useNavigate } from 'react-router-dom';
import {
  Container,
  Typography,
  Button,
  Box,
  Grid,
  Card,
  CardContent,
} from '@mui/material';
import ShoppingCartIcon from '@mui/icons-material/ShoppingCart';
import LocalShippingIcon from '@mui/icons-material/LocalShipping';
import VerifiedUserIcon from '@mui/icons-material/VerifiedUser';

function HomePage() {
  const navigate = useNavigate();

  const features = [
    {
      icon: <ShoppingCartIcon sx={{ fontSize: 60, color: 'primary.main' }} />,
      title: 'Easy Shopping',
      description: 'Browse and purchase products with just a few clicks',
    },
    {
      icon: <LocalShippingIcon sx={{ fontSize: 60, color: 'primary.main' }} />,
      title: 'Fast Delivery',
      description: 'Get your products delivered quickly and reliably',
    },
    {
      icon: <VerifiedUserIcon sx={{ fontSize: 60, color: 'primary.main' }} />,
      title: 'Secure Payment',
      description: 'Your transactions are protected with top-tier security',
    },
  ];

  return (
    <Container maxWidth="lg">
      <Box
        sx={{
          textAlign: 'center',
          py: 8,
        }}
      >
        <Typography
          variant="h2"
          component="h1"
          gutterBottom
          sx={{
            fontWeight: 'bold',
            mb: 3,
          }}
        >
          Welcome to Our Store
        </Typography>
        <Typography
          variant="h5"
          color="text.secondary"
          paragraph
          sx={{ mb: 4 }}
        >
          Discover amazing products at great prices
        </Typography>
        <Box sx={{ display: 'flex', gap: 2, justifyContent: 'center' }}>
          <Button
            variant="contained"
            size="large"
            onClick={() => navigate('/products')}
          >
            Shop Now
          </Button>
          <Button
            variant="outlined"
            size="large"
            onClick={() => navigate('/register')}
          >
            Sign Up
          </Button>
        </Box>
      </Box>

      <Box sx={{ py: 6 }}>
        <Typography
          variant="h4"
          component="h2"
          align="center"
          gutterBottom
          sx={{ mb: 4 }}
        >
          Why Choose Us
        </Typography>
        <Grid container spacing={4}>
          {features.map((feature, index) => (
            <Grid item xs={12} md={4} key={index}>
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
                <CardContent>
                  <Box sx={{ mb: 2 }}>{feature.icon}</Box>
                  <Typography variant="h5" component="h3" gutterBottom>
                    {feature.title}
                  </Typography>
                  <Typography variant="body1" color="text.secondary">
                    {feature.description}
                  </Typography>
                </CardContent>
              </Card>
            </Grid>
          ))}
        </Grid>
      </Box>
    </Container>
  );
}

export default HomePage;

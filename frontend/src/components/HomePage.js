import React from 'react';
import { Link } from 'react-router-dom';
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
          Welcome to CTO Parallel Test
        </Typography>
        <Typography variant="h5" component="h2" color="text.secondary" paragraph>
          Your one-stop shop for testing parallel development workflows
        </Typography>
        <Button
          variant="contained"
          color="primary"
          size="large"
          component={Link}
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

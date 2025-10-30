import React from 'react';
import { useParams } from 'react-router-dom';
import Container from '@mui/material/Container';
import Typography from '@mui/material/Typography';
import Box from '@mui/material/Box';
import Paper from '@mui/material/Paper';

function ProductDetail() {
  const { id } = useParams();

  return (
    <Container maxWidth="md">
      <Paper elevation={2} sx={{ p: 4, mt: 4 }}>
        <Typography variant="h4" component="h1" gutterBottom>
          Product Detail - ID: {id}
        </Typography>
        <Box sx={{ mt: 3 }}>
          <Typography variant="body1" color="text.secondary">
            This is a placeholder for the product detail page. In the future,
            this component will fetch and display detailed information about
            the product from the backend API.
          </Typography>
        </Box>
      </Paper>
    </Container>
  );
}

export default ProductDetail;

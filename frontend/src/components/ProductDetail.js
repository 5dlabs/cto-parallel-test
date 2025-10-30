import React from 'react';
import { useParams } from 'react-router-dom';
import Container from '@mui/material/Container';
import Typography from '@mui/material/Typography';
import Paper from '@mui/material/Paper';
import Box from '@mui/material/Box';

function ProductDetail() {
  const { id } = useParams();

  // TODO: Fetch product details from API using the id

  return (
    <Container maxWidth="md">
      <Paper elevation={3} sx={{ p: 4, mt: 4 }}>
        <Typography variant="h3" component="h1" gutterBottom>
          Product Detail - ID: {id}
        </Typography>
        <Box sx={{ mt: 3 }}>
          <Typography variant="body1" color="text.secondary">
            This is a placeholder for product details. In the future, this will display
            full product information including images, description, specifications, and
            customer reviews.
          </Typography>
        </Box>
      </Paper>
    </Container>
  );
}

export default ProductDetail;

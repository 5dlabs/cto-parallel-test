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
            This is a placeholder for full product information.
            Future implementation will fetch and display:
          </Typography>
          <Box component="ul" sx={{ mt: 2 }}>
            <Typography component="li">Product name and description</Typography>
            <Typography component="li">Product images</Typography>
            <Typography component="li">Price and availability</Typography>
            <Typography component="li">Product specifications</Typography>
            <Typography component="li">Customer reviews</Typography>
          </Box>
        </Box>
      </Paper>
    </Container>
  );
}

export default ProductDetail;

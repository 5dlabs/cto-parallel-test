import React from 'react';
import { useParams } from 'react-router-dom';
import Container from '@mui/material/Container';
import Typography from '@mui/material/Typography';
import Box from '@mui/material/Box';

function ProductDetail() {
  const { id } = useParams();

  return (
    <Container maxWidth="md">
      <Box sx={{ mt: 4 }}>
        <Typography variant="h3" component="h1" gutterBottom>
          Product Detail - ID: {id}
        </Typography>
        <Typography variant="body1" color="text.secondary" paragraph>
          This is a placeholder for product details. In the future, this will fetch and display
          detailed information about the product from the backend API.
        </Typography>
      </Box>
    </Container>
  );
}

export default ProductDetail;

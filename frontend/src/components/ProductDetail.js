import React from 'react';
import { useParams } from 'react-router-dom';
import Typography from '@mui/material/Typography';
import Box from '@mui/material/Box';
import Button from '@mui/material/Button';

function ProductDetail() {
  const { id } = useParams();

  return (
    <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
      <Typography variant="h4" component="h2" fontWeight={600}>
        Product Detail - ID: {id}
      </Typography>
      <Typography variant="body1" color="text.secondary">
        Detailed product information will appear here once connected to the backend service.
      </Typography>
      <Button variant="contained" color="primary" disabled>
        Add to Cart
      </Button>
    </Box>
  );
}

export default ProductDetail;

import React from 'react';
import { Box, Container, Typography } from '@mui/material';

const Footer = () => {
  const year = new Date().getFullYear();

  return (
    <Box component="footer" sx={{ bgcolor: 'grey.900', color: 'common.white', py: 3 }}>
      <Container maxWidth="lg" sx={{ textAlign: 'center' }}>
        <Typography variant="body2">
          &copy; {year} E-Shop. All rights reserved.
        </Typography>
      </Container>
    </Box>
  );
};

export default Footer;

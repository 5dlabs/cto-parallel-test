import React from 'react';
import { Box, Container, Typography } from '@mui/material';

function Footer() {
  const currentYear = new Date().getFullYear();

  return (
    <Box component="footer" sx={{ bgcolor: 'background.paper', py: 3, mt: 'auto' }}>
      <Container maxWidth="lg">
        <Typography variant="body2" color="text.secondary" align="center">
          © {currentYear} E-Commerce App. All rights reserved.
        </Typography>
      </Container>
    </Box>
  );
}

export default Footer;

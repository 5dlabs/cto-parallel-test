import React from 'react';
import { Box, Container, Typography } from '@mui/material';

const Footer = () => (
  <Box
    component="footer"
    py={3}
    textAlign="center"
    bgcolor="background.paper"
    sx={{ borderTop: 1, borderColor: 'divider', mt: 'auto' }}
  >
    <Container maxWidth="lg">
      <Typography variant="body2" color="text.secondary">
        © {new Date().getFullYear()} ShopSmart. All rights reserved.
      </Typography>
    </Container>
  </Box>
);

export default Footer;

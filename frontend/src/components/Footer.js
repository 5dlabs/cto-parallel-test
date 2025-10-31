import React from 'react';
import Box from '@mui/material/Box';
import Container from '@mui/material/Container';
import Typography from '@mui/material/Typography';

function Footer() {
  const currentYear = new Date().getFullYear();

  return (
    <Box component="footer" sx={{ bgcolor: 'primary.main', py: 3, color: 'primary.contrastText' }}>
      <Container maxWidth="lg">
        <Typography variant="body2" align="center">
          © {currentYear} ShopSmart. All rights reserved.
        </Typography>
      </Container>
    </Box>
  );
}

export default Footer;

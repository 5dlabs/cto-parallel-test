import React from "react";
import Box from "@mui/material/Box";
import Container from "@mui/material/Container";
import Typography from "@mui/material/Typography";

const Footer = () => (
  <Box component="footer" sx={{ bgcolor: "grey.100", py: 3, mt: 4 }}>
    <Container maxWidth="lg">
      <Typography variant="body2" color="text.secondary" align="center">
        &copy; {new Date().getFullYear()} ShopSmart. All rights reserved.
      </Typography>
    </Container>
  </Box>
);

export default Footer;

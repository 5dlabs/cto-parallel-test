import React from "react";
import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import Button from "@mui/material/Button";
import Stack from "@mui/material/Stack";
import { Link as RouterLink } from "react-router-dom";

const HomePage = () => (
  <Box sx={{ textAlign: "center", py: 6 }}>
    <Stack spacing={3} alignItems="center">
      <Typography variant="h3" component="h1" color="primary">
        Welcome to ShopSmart
      </Typography>
      <Typography variant="h6" color="text.secondary" maxWidth={600}>
        Discover the latest electronics, fashion, and home essentials at unbeatable prices.
      </Typography>
      <Button
        variant="contained"
        color="primary"
        size="large"
        component={RouterLink}
        to="/products"
      >
        Shop Now
      </Button>
    </Stack>
  </Box>
);

export default HomePage;

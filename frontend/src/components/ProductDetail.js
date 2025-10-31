import React from "react";
import { useParams } from "react-router-dom";
import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import Divider from "@mui/material/Divider";

const ProductDetail = () => {
  const { id } = useParams();

  return (
    <Box sx={{ maxWidth: 800, mx: "auto" }}>
      <Typography variant="h4" component="h2" gutterBottom>
        Product Detail - ID: {id}
      </Typography>
      <Divider sx={{ mb: 3 }} />
      <Typography variant="body1" color="text.secondary">
        Detailed product information will be displayed here in a future update.
      </Typography>
    </Box>
  );
};

export default ProductDetail;

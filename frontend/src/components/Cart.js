import React from "react";
import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import List from "@mui/material/List";
import ListItem from "@mui/material/ListItem";
import ListItemText from "@mui/material/ListItemText";

const Cart = () => (
  <Box sx={{ maxWidth: 700, mx: "auto" }}>
    <Typography variant="h4" component="h2" gutterBottom>
      Shopping Cart
    </Typography>
    <List>
      <ListItem>
        <ListItemText primary="Your cart is currently empty." secondary="Items added to your cart will appear here." />
      </ListItem>
    </List>
  </Box>
);

export default Cart;

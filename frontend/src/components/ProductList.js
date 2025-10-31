import React from "react";
import Grid from "@mui/material/Grid";
import Card from "@mui/material/Card";
import CardMedia from "@mui/material/CardMedia";
import CardContent from "@mui/material/CardContent";
import CardActions from "@mui/material/CardActions";
import Typography from "@mui/material/Typography";
import Button from "@mui/material/Button";
import Stack from "@mui/material/Stack";
import { Link as RouterLink } from "react-router-dom";

const products = [
  {
    id: 1,
    name: "Wireless Headphones",
    price: "$199.99",
    description: "Experience immersive sound with our premium noise-cancelling headphones.",
  },
  {
    id: 2,
    name: "Smartwatch Pro",
    price: "$249.99",
    description: "Track your fitness goals and stay connected with our latest smartwatch.",
  },
  {
    id: 3,
    name: "4K Ultra HD TV",
    price: "$899.99",
    description: "Bring the cinema home with stunning visuals and vibrant colors.",
  },
];

const ProductList = () => (
  <Stack spacing={4}>
    <Typography variant="h4" component="h2">
      Featured Products
    </Typography>
    <Grid container spacing={4}>
      {products.map((product) => (
        <Grid item xs={12} sm={6} md={4} key={product.id}>
          <Card sx={{ height: "100%", display: "flex", flexDirection: "column" }}>
            <CardMedia
              component="img"
              height="180"
              image="https://via.placeholder.com/400x300.png?text=Product"
              alt={product.name}
            />
            <CardContent sx={{ flexGrow: 1 }}>
              <Typography gutterBottom variant="h6" component="h3">
                {product.name}
              </Typography>
              <Typography variant="subtitle1" color="secondary" gutterBottom>
                {product.price}
              </Typography>
              <Typography variant="body2" color="text.secondary">
                {product.description}
              </Typography>
            </CardContent>
            <CardActions sx={{ justifyContent: "space-between", px: 2, pb: 2 }}>
              <Button
                size="small"
                variant="outlined"
                component={RouterLink}
                to={`/products/${product.id}`}
              >
                View Details
              </Button>
              <Button size="small" variant="contained" color="secondary">
                Add to Cart
              </Button>
            </CardActions>
          </Card>
        </Grid>
      ))}
    </Grid>
  </Stack>
);

export default ProductList;

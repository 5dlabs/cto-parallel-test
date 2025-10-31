import React from "react";
import AppBar from "@mui/material/AppBar";
import Toolbar from "@mui/material/Toolbar";
import Typography from "@mui/material/Typography";
import Button from "@mui/material/Button";
import IconButton from "@mui/material/IconButton";
import Badge from "@mui/material/Badge";
import Stack from "@mui/material/Stack";
import { Link as RouterLink } from "react-router-dom";
import ShoppingCartIcon from "@mui/icons-material/ShoppingCart";

const Header = () => {
  const isAuthenticated = false;
  const cartCount = 0;

  return (
    <AppBar position="static" color="primary">
      <Toolbar>
        <Typography
          variant="h6"
          component={RouterLink}
          to="/"
          sx={{ flexGrow: 1, color: "inherit", textDecoration: "none" }}
        >
          ShopSmart
        </Typography>
        <Stack direction="row" spacing={2} alignItems="center">
          <Button component={RouterLink} to="/products" color="inherit">
            Products
          </Button>
          <IconButton
            component={RouterLink}
            to="/cart"
            color="inherit"
            aria-label="View cart"
          >
            <Badge badgeContent={cartCount} color="secondary">
              <ShoppingCartIcon />
            </Badge>
          </IconButton>
          <Button
            component={RouterLink}
            to={isAuthenticated ? "/" : "/login"}
            color="inherit"
          >
            {isAuthenticated ? "Logout" : "Login"}
          </Button>
        </Stack>
      </Toolbar>
    </AppBar>
  );
};

export default Header;

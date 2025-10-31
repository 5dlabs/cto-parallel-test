import React from "react";
import Box from "@mui/material/Box";
import TextField from "@mui/material/TextField";
import Button from "@mui/material/Button";
import Typography from "@mui/material/Typography";
import Stack from "@mui/material/Stack";

const Login = () => (
  <Box sx={{ maxWidth: 400, mx: "auto" }}>
    <Typography variant="h4" component="h2" gutterBottom>
      Login
    </Typography>
    <Stack component="form" spacing={3} noValidate>
      <TextField label="Email" type="email" required fullWidth autoComplete="email" />
      <TextField label="Password" type="password" required fullWidth autoComplete="current-password" />
      <Button type="submit" variant="contained" color="primary" fullWidth>
        Sign In
      </Button>
    </Stack>
  </Box>
);

export default Login;

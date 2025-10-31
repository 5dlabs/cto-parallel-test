import React from "react";
import Box from "@mui/material/Box";
import TextField from "@mui/material/TextField";
import Button from "@mui/material/Button";
import Typography from "@mui/material/Typography";
import Stack from "@mui/material/Stack";

const Register = () => (
  <Box sx={{ maxWidth: 400, mx: "auto" }}>
    <Typography variant="h4" component="h2" gutterBottom>
      Create Account
    </Typography>
    <Stack component="form" spacing={3} noValidate>
      <TextField label="Full Name" required fullWidth autoComplete="name" />
      <TextField label="Email" type="email" required fullWidth autoComplete="email" />
      <TextField label="Password" type="password" required fullWidth autoComplete="new-password" />
      <TextField label="Confirm Password" type="password" required fullWidth autoComplete="new-password" />
      <Button type="submit" variant="contained" color="primary" fullWidth>
        Register
      </Button>
    </Stack>
  </Box>
);

export default Register;

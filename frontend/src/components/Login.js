import React from 'react';
import { Paper, Typography, Stack, TextField, Button } from '@mui/material';

const Login = () => (
  <Paper
    component="form"
    noValidate
    autoComplete="off"
    sx={{ p: { xs: 3, md: 4 }, maxWidth: 480, mx: 'auto' }}
    elevation={2}
  >
    <Stack spacing={3}>
      <Typography variant="h4" component="h1" textAlign="center">
        Login
      </Typography>
      <TextField label="Email" type="email" required fullWidth />
      <TextField label="Password" type="password" required fullWidth />
      <Button type="submit" variant="contained" color="primary" size="large">
        Sign In
      </Button>
    </Stack>
  </Paper>
);

export default Login;

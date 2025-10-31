import React from 'react';
import { Container, Typography, TextField, Button, Stack, Paper } from '@mui/material';

const Login = () => (
  <Container maxWidth="sm">
    <Paper elevation={3} sx={{ p: 4 }}>
      <Stack component="form" spacing={3} noValidate>
        <Typography variant="h4" component="h1" sx={{ fontWeight: 600, textAlign: 'center' }}>
          Login
        </Typography>
        <TextField label="Email" type="email" required fullWidth autoComplete="email" />
        <TextField label="Password" type="password" required fullWidth autoComplete="current-password" />
        <Button type="submit" variant="contained" color="primary" size="large">
          Sign In
        </Button>
      </Stack>
    </Paper>
  </Container>
);

export default Login;

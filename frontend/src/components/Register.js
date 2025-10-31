import React from 'react';
import { Container, Typography, TextField, Button, Stack, Paper } from '@mui/material';

const Register = () => (
  <Container maxWidth="sm">
    <Paper elevation={3} sx={{ p: 4 }}>
      <Stack component="form" spacing={3} noValidate>
        <Typography variant="h4" component="h1" sx={{ fontWeight: 600, textAlign: 'center' }}>
          Create Account
        </Typography>
        <TextField label="First Name" required fullWidth autoComplete="given-name" />
        <TextField label="Last Name" required fullWidth autoComplete="family-name" />
        <TextField label="Email" type="email" required fullWidth autoComplete="email" />
        <TextField label="Password" type="password" required fullWidth autoComplete="new-password" />
        <Button type="submit" variant="contained" color="primary" size="large">
          Register
        </Button>
      </Stack>
    </Paper>
  </Container>
);

export default Register;

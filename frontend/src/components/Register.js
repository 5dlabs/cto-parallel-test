import React from 'react';
import { Paper, Typography, Stack, TextField, Button } from '@mui/material';

const Register = () => (
  <Paper
    component="form"
    noValidate
    autoComplete="off"
    sx={{ p: { xs: 3, md: 4 }, maxWidth: 520, mx: 'auto' }}
    elevation={2}
  >
    <Stack spacing={3}>
      <Typography variant="h4" component="h1" textAlign="center">
        Create Account
      </Typography>
      <TextField label="Full Name" required fullWidth />
      <TextField label="Email" type="email" required fullWidth />
      <TextField label="Password" type="password" required fullWidth />
      <TextField label="Confirm Password" type="password" required fullWidth />
      <Button type="submit" variant="contained" color="primary" size="large">
        Register
      </Button>
    </Stack>
  </Paper>
);

export default Register;

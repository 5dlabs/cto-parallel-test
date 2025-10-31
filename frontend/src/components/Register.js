import React from 'react';
import Box from '@mui/material/Box';
import TextField from '@mui/material/TextField';
import Button from '@mui/material/Button';
import Typography from '@mui/material/Typography';
import Paper from '@mui/material/Paper';

function Register() {
  const handleSubmit = (event) => {
    event.preventDefault();
  };

  return (
    <Paper elevation={1} sx={{ p: { xs: 3, md: 5 }, maxWidth: 480, mx: 'auto' }}>
      <Typography variant="h4" component="h2" fontWeight={600} gutterBottom>
        Create Account
      </Typography>
      <Box component="form" onSubmit={handleSubmit} sx={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
        <TextField label="Full Name" required fullWidth autoComplete="name" />
        <TextField label="Email" type="email" required fullWidth autoComplete="email" />
        <TextField label="Password" type="password" required fullWidth autoComplete="new-password" />
        <Button type="submit" variant="contained" color="primary">
          Register
        </Button>
      </Box>
    </Paper>
  );
}

export default Register;

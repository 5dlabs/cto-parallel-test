import React from 'react';
import { Link } from 'react-router-dom';
import Container from '@mui/material/Container';
import Paper from '@mui/material/Paper';
import Typography from '@mui/material/Typography';
import TextField from '@mui/material/TextField';
import Button from '@mui/material/Button';
import Box from '@mui/material/Box';

function Register() {
  const handleSubmit = (event) => {
    event.preventDefault();
    // TODO: Connect to user registration API
    console.log('Registration form submitted');
  };

  return (
    <Container maxWidth="sm">
      <Paper elevation={3} sx={{ p: 4, mt: 4 }}>
        <Typography variant="h4" component="h1" gutterBottom align="center">
          Register
        </Typography>
        <Box component="form" onSubmit={handleSubmit} sx={{ mt: 3 }}>
          <TextField
            fullWidth
            label="Username"
            type="text"
            name="username"
            margin="normal"
            required
          />
          <TextField
            fullWidth
            label="Email"
            type="email"
            name="email"
            margin="normal"
            required
          />
          <TextField
            fullWidth
            label="Password"
            type="password"
            name="password"
            margin="normal"
            required
          />
          <TextField
            fullWidth
            label="Confirm Password"
            type="password"
            name="confirmPassword"
            margin="normal"
            required
          />
          <Button
            type="submit"
            fullWidth
            variant="contained"
            size="large"
            sx={{ mt: 3, mb: 2 }}
          >
            Register
          </Button>
          <Box sx={{ textAlign: 'center' }}>
            <Typography variant="body2">
              Already have an account?{' '}
              <Link to="/login" style={{ textDecoration: 'none' }}>
                Login here
              </Link>
            </Typography>
          </Box>
        </Box>
      </Paper>
    </Container>
  );
}

export default Register;

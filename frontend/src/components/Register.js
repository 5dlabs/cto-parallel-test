import Container from '@mui/material/Container';
import Typography from '@mui/material/Typography';
import TextField from '@mui/material/TextField';
import Button from '@mui/material/Button';
import Stack from '@mui/material/Stack';
import Link from '@mui/material/Link';
import Box from '@mui/material/Box';
import { Link as RouterLink } from 'react-router-dom';

const Register = () => (
  <Container maxWidth="sm" sx={{ mt: 6 }}>
    <Stack spacing={4}>
      <Typography variant="h4" component="h1" fontWeight={600} textAlign="center">
        Create Account
      </Typography>
      <Box component="form" noValidate>
        <Stack spacing={3}>
          <TextField label="Full Name" fullWidth required />
          <TextField label="Email" type="email" fullWidth required />
          <TextField label="Password" type="password" fullWidth required />
          <TextField label="Confirm Password" type="password" fullWidth required />
          <Button type="submit" variant="contained" color="primary" size="large">
            Register
          </Button>
        </Stack>
      </Box>
      <Typography variant="body2" textAlign="center">
        Already have an account?{' '}
        <Link component={RouterLink} to="/login" underline="hover">
          Sign in
        </Link>
      </Typography>
    </Stack>
  </Container>
);

export default Register;

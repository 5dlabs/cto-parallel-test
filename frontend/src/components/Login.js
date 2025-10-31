import Container from '@mui/material/Container';
import Typography from '@mui/material/Typography';
import TextField from '@mui/material/TextField';
import Button from '@mui/material/Button';
import Stack from '@mui/material/Stack';
import Link from '@mui/material/Link';
import Box from '@mui/material/Box';
import { Link as RouterLink } from 'react-router-dom';

const Login = () => (
  <Container maxWidth="sm" sx={{ mt: 6 }}>
    <Stack spacing={4}>
      <Typography variant="h4" component="h1" fontWeight={600} textAlign="center">
        Login
      </Typography>
      <Box component="form" noValidate>
        <Stack spacing={3}>
          <TextField label="Email" type="email" fullWidth required />
          <TextField label="Password" type="password" fullWidth required />
          <Button type="submit" variant="contained" color="primary" size="large">
            Sign In
          </Button>
        </Stack>
      </Box>
      <Typography variant="body2" textAlign="center">
        Don't have an account?{' '}
        <Link component={RouterLink} to="/register" underline="hover">
          Register now
        </Link>
      </Typography>
    </Stack>
  </Container>
);

export default Login;

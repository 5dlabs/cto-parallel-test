import Box from '@mui/material/Box';
import Container from '@mui/material/Container';
import Typography from '@mui/material/Typography';

const Footer = () => {
  const currentYear = new Date().getFullYear();

  return (
    <Box component="footer" sx={{ py: 4, backgroundColor: 'primary.main', color: 'primary.contrastText' }}>
      <Container maxWidth="lg" sx={{ textAlign: 'center' }}>
        <Typography variant="body2">&copy; {currentYear} ShopSmart. All rights reserved.</Typography>
      </Container>
    </Box>
  );
};

export default Footer;

import { render, screen } from '@testing-library/react';
import App from './App';

test('renders without crashing', () => {
  render(<App />);
  // Check if the header with E-Commerce text is present
  const linkElement = screen.getByText(/E-Commerce/i);
  expect(linkElement).toBeInTheDocument();
});

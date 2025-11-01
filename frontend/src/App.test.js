import { render, screen } from '@testing-library/react';
import App from './App';

test('renders e-commerce app', () => {
  render(<App />);
  const welcomeElement = screen.getByText(/Welcome to E-Commerce/i);
  expect(welcomeElement).toBeInTheDocument();
});

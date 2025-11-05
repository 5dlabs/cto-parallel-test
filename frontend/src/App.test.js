import { render, screen } from '@testing-library/react';
import App from './App';

test('renders without crashing', () => {
  render(<App />);
  // Basic smoke test - just verify the app renders
  expect(document.querySelector('.min-h-screen')).toBeInTheDocument();
});

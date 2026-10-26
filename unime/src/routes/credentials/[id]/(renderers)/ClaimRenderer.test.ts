import '@testing-library/jest-dom';

import { render, screen } from '@testing-library/svelte';

import ClaimRenderer from './ClaimRenderer.svelte';

describe('ClaimRenderer', () => {
  test('renders a timestamp in a human-readable format', () => {
    render(ClaimRenderer, { key: 'memberSince', value: '2019-11-05T14:30:00Z' });
    expect(screen.getByText(/November 5, 2019 at/)).toBeInTheDocument();
  });

  test('renders a timestamp without a time of day as a date', () => {
    render(ClaimRenderer, { key: 'birthdate', value: '1985-05-21' });
    expect(screen.getByText('May 21, 1985')).toBeInTheDocument();
  });

  test('renders a country code as the name of the country', () => {
    render(ClaimRenderer, { key: 'countryOfBirth', value: 'DE' });
    expect(screen.getByText('Germany')).toBeInTheDocument();
  });

  test('renders anything else as plain text', () => {
    render(ClaimRenderer, { key: 'givenName', value: 'Ferris' });
    expect(screen.getByText('Ferris')).toBeInTheDocument();
  });

  test('renders the domain of a website claim as a link to its HTTPS URL', () => {
    render(ClaimRenderer, { key: 'website', value: 'impierce.com' });
    expect(screen.getByRole('link', { name: 'impierce.com' })).toHaveAttribute('href', 'https://impierce.com/');
  });

  test('does not render a domain-like name as a link', () => {
    render(ClaimRenderer, { key: 'givenName', value: 'Ferris.Crab' });
    expect(screen.queryByRole('link')).not.toBeInTheDocument();
  });
});

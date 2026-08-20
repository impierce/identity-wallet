import '@testing-library/jest-dom';

import { render, screen } from '@testing-library/svelte';

import InlineClaimRenderer from './InlineClaimRenderer.svelte';

describe('InlineClaimRenderer', () => {
  test('renders the name and the value of a claim next to each other', () => {
    render(InlineClaimRenderer, { key: 'givenName', value: 'Ferris' });
    expect(screen.getByText('givenName')).toBeInTheDocument();
    expect(screen.getByText('Ferris')).toBeInTheDocument();
  });

  test('renders a timestamp in a human-readable format', () => {
    render(InlineClaimRenderer, { key: 'dateOfBirth', value: '1985-05-21' });
    expect(screen.getByText('May 21, 1985')).toBeInTheDocument();
  });

  test('renders a country code as the name of the country', () => {
    render(InlineClaimRenderer, { key: 'country', value: 'NL' });
    expect(screen.getByText('Netherlands')).toBeInTheDocument();
  });

  test('renders values that are not strings', () => {
    render(InlineClaimRenderer, { key: 'creditsEarned', value: 5 });
    expect(screen.getByText('5')).toBeInTheDocument();
  });
});

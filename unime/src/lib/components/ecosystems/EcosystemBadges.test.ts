import '@testing-library/jest-dom';

import { setLocale } from '$i18n/i18n-svelte';
import { loadAllLocales } from '$i18n/i18n-util.sync';

import type { EcosystemProfile } from '@bindings/user_prompt/EcosystemProfile';
import { render, screen } from '@testing-library/svelte';

import EcosystemBadges from './EcosystemBadges.svelte';

const ecosystem = (name: string): EcosystemProfile => ({
  logo_uri: null,
  name,
  description: null,
  ecosystem_leader: { logo_uri: null, name, description: null, identifier: null },
  member_count: 0,
  members: [],
  trust_chain: [],
});

describe('EcosystemBadges', () => {
  beforeAll(() => {
    loadAllLocales();
    setLocale('en');
  });

  test('shows a mark per ecosystem and names them all', () => {
    render(EcosystemBadges, { ecosystems: [ecosystem('Global FinTech Alliance'), ecosystem('Open Skills')] });

    expect(screen.getByRole('img')).toHaveAccessibleName('Endorsed by Global FinTech Alliance, Open Skills');
    expect(screen.getByText('GFA')).toBeInTheDocument();
    expect(screen.getByText('OS')).toBeInTheDocument();
  });

  test('counts the ecosystems beyond the second', () => {
    render(EcosystemBadges, {
      ecosystems: [ecosystem('Alpha'), ecosystem('Beta'), ecosystem('Gamma'), ecosystem('Delta')],
    });

    expect(screen.getByText('+2')).toBeInTheDocument();
    expect(screen.queryByText('GA')).not.toBeInTheDocument();
  });

  test('renders nothing without ecosystems', () => {
    render(EcosystemBadges, { ecosystems: [] });

    expect(screen.queryByRole('img')).not.toBeInTheDocument();
  });
});

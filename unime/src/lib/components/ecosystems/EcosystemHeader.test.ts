import '@testing-library/jest-dom';

import { setLocale } from '$i18n/i18n-svelte';
import { loadAllLocales } from '$i18n/i18n-util.sync';

import type { EcosystemProfile } from '@bindings/user_prompt/EcosystemProfile';
import { render, screen } from '@testing-library/svelte';

import EcosystemHeader from './EcosystemHeader.svelte';

const ecosystem = (name: string, leaderName: string): EcosystemProfile => ({
  logo_uri: null,
  name,
  description: null,
  ecosystem_leader: { logo_uri: null, name: leaderName, description: null, identifier: null },
  member_count: 3,
  members: [],
  trust_chain: [],
});

describe('EcosystemHeader', () => {
  beforeAll(() => {
    loadAllLocales();
    setLocale('en');
  });

  test('shows the ecosystem name with its leader below it', () => {
    render(EcosystemHeader, { ecosystem: ecosystem('Community Learning Network', 'Capella'), isTempAsset: false });

    const name = screen.getByText('Community Learning Network');
    const leader = screen.getByText('Led by Capella');
    expect(name.compareDocumentPosition(leader) & Node.DOCUMENT_POSITION_FOLLOWING).toBeTruthy();
    expect(screen.getByText('3 Members')).toBeInTheDocument();
  });

  test('leaves out a leader that only repeats the ecosystem name', () => {
    render(EcosystemHeader, { ecosystem: ecosystem('Capella', 'Capella'), isTempAsset: false });

    expect(screen.getByText('Capella')).toBeInTheDocument();
    expect(screen.queryByText(/Led by/)).not.toBeInTheDocument();
  });
});

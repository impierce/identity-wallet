import '@testing-library/jest-dom';

import { setLocale } from '$i18n/i18n-svelte';
import { loadAllLocales } from '$i18n/i18n-util.sync';

import type { Connection } from '@bindings/connections/Connection';
import type { EcosystemsCheck } from '@bindings/connections/EcosystemsCheck';
import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';
import type { EcosystemProfile } from '@bindings/user_prompt/EcosystemProfile';
import type { Member } from '@bindings/user_prompt/Member';
import { fireEvent, render, screen } from '@testing-library/svelte';

import { state } from '$lib/stores';

import CredentialEcosystems from './CredentialEcosystems.svelte';

const member = (name: string, identifier: string): Member => ({
  logo_uri: null,
  name,
  description: null,
  identifier,
});

const ecosystem: EcosystemProfile = {
  logo_uri: null,
  name: 'Dutch Organization for Universities',
  description: null,
  ecosystem_leader: member('Universities Foundation', 'https://universities.example'),
  member_count: 2,
  members: [],
  trust_chain: [
    member('University of Harderwijk', 'https://university.example'),
    member('Regional Education Council', 'https://council.example'),
    member('Universities Foundation', 'https://universities.example'),
  ],
};

const credential = { id: 'credential', connection_id: 'issuer' } as DisplayCredential;

function withIssuer(ecosystems: EcosystemProfile[], ecosystems_check?: EcosystemsCheck) {
  state.update((appState) => ({
    ...appState,
    connections: [{ id: 'issuer', ecosystems, ecosystems_check } as Connection],
  }));
}

describe('CredentialEcosystems', () => {
  beforeAll(() => {
    loadAllLocales();
    setLocale('en');
  });

  test('lists the endorsing ecosystems and expands one into its trust chain', async () => {
    withIssuer([ecosystem], { resolved: true, last_checked: '2026-11-02T10:00:00Z' });
    render(CredentialEcosystems, { credential });

    expect(screen.getByText('Endorsed by')).toBeInTheDocument();
    expect(screen.queryByTestId('trust-chain')).not.toBeInTheDocument();
    expect(screen.queryByTestId('ecosystems-unconfirmed')).not.toBeInTheDocument();

    await fireEvent.click(
      screen.getByRole('button', { name: 'Show the trust chain of Dutch Organization for Universities' }),
    );

    const chain = screen.getByTestId('trust-chain');
    expect(chain).toHaveTextContent('University of Harderwijk');
    expect(chain).toHaveTextContent('Issuer · university.example');
    expect(chain).toHaveTextContent('Regional Education Council');
    expect(chain).toHaveTextContent('Ecosystem leader · universities.example');
  });

  test('warns when the endorsements could not be confirmed, but keeps showing them', () => {
    withIssuer([ecosystem], {
      resolved: false,
      last_checked: '2026-11-02T10:00:00Z',
      last_resolved: '2026-10-01T10:00:00Z',
    });
    render(CredentialEcosystems, { credential });

    expect(screen.getByTestId('ecosystems-unconfirmed')).toHaveTextContent('could not be confirmed just now');
    expect(screen.getByText('Dutch Organization for Universities')).toBeInTheDocument();
  });

  test('warns when it is unknown whether the issuer is endorsed', () => {
    withIssuer([], { resolved: false, last_checked: '2026-11-02T10:00:00Z' });
    render(CredentialEcosystems, { credential });

    expect(screen.getByTestId('ecosystems-unconfirmed')).toHaveTextContent(
      'Could not check whether the issuer is endorsed by an ecosystem.',
    );
    expect(screen.queryByText('Endorsed by')).not.toBeInTheDocument();
  });

  test('shows standard issuer details only when the issuer is endorsed by none', () => {
    withIssuer([], { resolved: true, last_checked: '2026-11-02T10:00:00Z' });
    render(CredentialEcosystems, { credential });

    expect(screen.queryByText('Endorsed by')).not.toBeInTheDocument();
    expect(screen.queryByTestId('ecosystems-unconfirmed')).not.toBeInTheDocument();
  });

  test('asks for a refresh, and shows that one is running', async () => {
    withIssuer([ecosystem]);
    const onRefresh = vi.fn();
    const { rerender } = render(CredentialEcosystems, { props: { credential }, events: { refresh: onRefresh } });

    await fireEvent.click(screen.getByTestId('credential-refresh'));
    expect(onRefresh).toHaveBeenCalledOnce();

    await rerender({ credential, refreshing: true });
    expect(screen.getByTestId('credential-refresh')).toBeDisabled();
    expect(screen.getByTestId('credential-refresh')).toHaveTextContent('Updating…');
  });
});

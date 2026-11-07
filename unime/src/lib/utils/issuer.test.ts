import type { Connection } from '@bindings/connections/Connection';
import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';

import { currentConnectionName, currentHistoryIssuerName, currentIssuerName } from './issuer';

const connection = (id: string, name: string): Connection => ({
  id,
  name,
  url: 'https://mira.example.com',
  did: 'did:web:mira.example.com',
  verified: false,
  first_interacted: '2026-10-28T08:00:00Z',
  last_interacted: '2026-11-03T08:00:00Z',
});

const credential = (id: string, issuer_name: string, connection_id?: string) =>
  ({ id, issuer_name, connection_id, data: { issuer: { name: 'Name in the credential data' } } }) as DisplayCredential;

const connections = [connection('mira', 'Mira Renamed')];

describe('currentIssuerName', () => {
  test('uses the current name of the connection over the name at issuance', () => {
    expect(currentIssuerName(credential('1', 'Mira', 'mira'), connections)).toBe('Mira Renamed');
  });

  test('falls back to the name at issuance without a connection', () => {
    expect(currentIssuerName(credential('1', 'Mira'), connections)).toBe('Mira');
  });

  test('falls back to the name at issuance when the connection is gone', () => {
    expect(currentIssuerName(credential('1', 'Mira', 'deleted'), connections)).toBe('Mira');
  });

  test('falls back to the issuer in the credential data without an issuer name', () => {
    const withoutIssuerName = { ...credential('1', 'Mira'), issuer_name: undefined } as unknown as DisplayCredential;
    expect(currentIssuerName(withoutIssuerName, [])).toBe('Name in the credential data');
  });
});

describe('currentConnectionName', () => {
  test('uses the current name of the connection', () => {
    expect(currentConnectionName('mira', 'Mira', connections)).toBe('Mira Renamed');
  });

  test('keeps the name of the event when the connection is gone', () => {
    expect(currentConnectionName('deleted', 'Mira', connections)).toBe('Mira');
  });
});

describe('currentHistoryIssuerName', () => {
  const historyCredential = { id: '1', title: 'PID', issuer_name: 'Mira' };

  test('uses the current issuer name of a credential that is still in the wallet', () => {
    expect(currentHistoryIssuerName(historyCredential, [credential('1', 'Mira', 'mira')], connections)).toBe(
      'Mira Renamed',
    );
  });

  test('keeps the name of the event for a deleted credential', () => {
    expect(currentHistoryIssuerName(historyCredential, [], connections)).toBe('Mira');
  });
});

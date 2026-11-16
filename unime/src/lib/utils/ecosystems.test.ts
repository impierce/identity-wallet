import type { Connection } from '@bindings/connections/Connection';
import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';
import type { EcosystemProfile } from '@bindings/user_prompt/EcosystemProfile';

import { issuerEcosystems } from './ecosystems';

const ecosystem = { name: 'Ecosystem One' } as EcosystemProfile;
const check = { resolved: false, last_checked: '2026-11-02T10:00:00Z' };

const connection = {
  id: 'issuer',
  ecosystems: [ecosystem],
  ecosystems_check: check,
} as Connection;

const credential = (connection_id?: string) => ({ connection_id }) as DisplayCredential;

describe('issuerEcosystems', () => {
  test('returns the ecosystems and check of the issuer connection', () => {
    expect(issuerEcosystems(credential('issuer'), [connection])).toEqual({ ecosystems: [ecosystem], check });
  });

  test('returns no ecosystems for a credential without an issuer connection', () => {
    expect(issuerEcosystems(credential(), [connection])).toEqual({ ecosystems: [], check: undefined });
  });

  test('returns no ecosystems when the issuer connection was deleted', () => {
    expect(issuerEcosystems(credential('deleted'), [connection])).toEqual({ ecosystems: [], check: undefined });
  });
});

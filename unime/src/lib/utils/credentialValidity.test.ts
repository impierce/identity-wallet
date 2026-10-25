import type { CredentialStatus } from '@bindings/credentials/CredentialStatus';

import { getCredentialValidity } from './credentialValidity';

const now = new Date('2026-06-01T00:00:00Z');

function credential(
  date_issued: string,
  expiration_date?: string,
  status?: CredentialStatus['status'],
): Parameters<typeof getCredentialValidity>[0] {
  return {
    metadata: { is_favorite: false, date_added: '2026-01-01T00:00:00Z', date_issued, expiration_date },
    credential_status: status ? { status, last_checked: '2026-01-01T00:00:00Z' } : undefined,
  };
}

describe('getCredentialValidity', () => {
  test('is valid until the expiration date', () => {
    expect(getCredentialValidity(credential('2025-01-01T00:00:00Z', '2040-11-03T00:00:00Z'), now)).toEqual({
      state: 'valid',
      validUntil: '2040-11-03T00:00:00Z',
    });
  });

  test('is valid without an end date when there is no expiration date', () => {
    expect(getCredentialValidity(credential('2025-01-01T00:00:00Z'), now)).toEqual({ state: 'valid' });
  });

  test('is expired once the expiration date has passed', () => {
    expect(getCredentialValidity(credential('2020-01-01T00:00:00Z', '2024-11-03T00:00:00Z'), now)).toEqual({
      state: 'expired',
      validUntil: '2024-11-03T00:00:00Z',
    });
  });

  test('is not valid yet before the valid-from date', () => {
    expect(getCredentialValidity(credential('2027-01-01T00:00:00Z', '2032-01-01T00:00:00Z'), now)).toEqual({
      state: 'not_yet_valid',
      validFrom: '2027-01-01T00:00:00Z',
    });
  });

  test('revocation wins over the validity period', () => {
    expect(getCredentialValidity(credential('2020-01-01T00:00:00Z', '2024-11-03T00:00:00Z', 'INVALID'), now)).toEqual({
      state: 'revoked',
    });
    expect(getCredentialValidity(credential('2027-01-01T00:00:00Z', undefined, 'INVALID'), now)).toEqual({
      state: 'revoked',
    });
  });

  test('is suspended when the status list says so', () => {
    expect(getCredentialValidity(credential('2025-01-01T00:00:00Z', undefined, 'SUSPENDED'), now)).toEqual({
      state: 'suspended',
    });
  });

  test('ignores missing or unparsable dates', () => {
    expect(getCredentialValidity(credential('', 'not a date', 'UNDEFINED'), now)).toEqual({ state: 'valid' });
  });
});

import { getDisplayClaimKey } from './displayClaim';

describe('getDisplayClaimKey', () => {
  test('uses the final claim path element when display metadata has no name', () => {
    expect(
      getDisplayClaimKey({
        path: ['credentialSubject', 'email'],
        key: '',
        value: 'ferris@example.com',
      }),
    ).toBe('email');
  });

  test('prefers the issuer-provided display name', () => {
    expect(
      getDisplayClaimKey({
        path: ['credentialSubject', 'email'],
        key: 'Email address',
        value: 'ferris@example.com',
      }),
    ).toBe('Email address');
  });
});

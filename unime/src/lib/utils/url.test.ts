import { describe, expect, test } from 'vitest';

import { connectionBrowserUrl, connectionHostname } from './url';

describe('connectionHostname', () => {
  test('removes an identity subdomain prefix from the displayed hostname', () => {
    expect(connectionHostname('https://identity.example.com/connect')).toBe('example.com');
  });

  test('accepts legacy connection URLs without a scheme', () => {
    expect(connectionHostname('identity.example.com')).toBe('example.com');
  });

  test('does not remove identity when it is not the complete first label', () => {
    expect(connectionHostname('https://identity-provider.example.com')).toBe('identity-provider.example.com');
  });
});

describe('connectionBrowserUrl', () => {
  test('preserves a complete HTTPS connection URL', () => {
    expect(connectionBrowserUrl('https://identity.example.com/connect')).toBe('https://identity.example.com/connect');
  });

  test('adds HTTPS to a legacy connection URL without a scheme', () => {
    expect(connectionBrowserUrl('example.com')).toBe('https://example.com/');
  });

  test('rejects non-browser URL schemes', () => {
    expect(connectionBrowserUrl('javascript:alert(1)')).toBeUndefined();
  });
});

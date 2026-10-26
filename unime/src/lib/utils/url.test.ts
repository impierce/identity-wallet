import { describe, expect, test } from 'vitest';

import { connectionHostname, domainUrl, linkifyClaim } from './url';

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

describe('domainUrl', () => {
  test('turns a bare hostname into an HTTPS URL', () => {
    expect(domainUrl('impierce.com')).toBe('https://impierce.com/');
  });

  test('rejects values that are not a bare hostname', () => {
    expect(domainUrl('https://impierce.com')).toBeUndefined();
    expect(domainUrl('impierce.com/about')).toBeUndefined();
    expect(domainUrl('localhost')).toBeUndefined();
    expect(domainUrl('1.2.3.4')).toBeUndefined();
    expect(domainUrl('javascript:alert(1)')).toBeUndefined();
  });
});

describe('linkifyClaim', () => {
  test('links HTTPS URLs inside text', () => {
    expect(linkifyClaim('description', 'See https://impierce.com/about for more')).toEqual([
      { text: 'See ' },
      { text: 'https://impierce.com/about', url: 'https://impierce.com/about' },
      { text: ' for more' },
    ]);
  });

  test('leaves trailing punctuation out of a link', () => {
    expect(linkifyClaim('description', 'Visit https://impierce.com.')).toEqual([
      { text: 'Visit ' },
      { text: 'https://impierce.com', url: 'https://impierce.com' },
      { text: '.' },
    ]);
  });

  test('does not link other URL schemes', () => {
    expect(linkifyClaim('url', 'http://impierce.com')).toEqual([{ text: 'http://impierce.com' }]);
    expect(linkifyClaim('url', 'javascript:alert(1)')).toEqual([{ text: 'javascript:alert(1)' }]);
  });

  test('links a bare domain in a website claim', () => {
    expect(linkifyClaim('Website', 'impierce.com')).toEqual([{ text: 'impierce.com', url: 'https://impierce.com/' }]);
    expect(linkifyClaim('url', 'impierce.com')).toEqual([{ text: 'impierce.com', url: 'https://impierce.com/' }]);
  });

  test('links a bare domain starting with www in any claim', () => {
    expect(linkifyClaim('organization', 'www.impierce.com')).toEqual([
      { text: 'www.impierce.com', url: 'https://www.impierce.com/' },
    ]);
  });

  test('does not link domain-like values in other claims', () => {
    expect(linkifyClaim('name', 'Jane.Doe')).toEqual([{ text: 'Jane.Doe' }]);
    expect(linkifyClaim('file', 'readme.md')).toEqual([{ text: 'readme.md' }]);
    expect(linkifyClaim('description', 'Works at impierce.com')).toEqual([{ text: 'Works at impierce.com' }]);
  });
});

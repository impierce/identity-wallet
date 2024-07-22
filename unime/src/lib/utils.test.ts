import { calculateInitials, formatDate, hash } from './utils';

describe('hash function', () => {
  test('should return the expected hash digest', () => {
    expect(hash('foobar')).toEqual('c3ab8ff13720e8ad9047dd39466b3c8974e592c2fa383d4a3960714caef0c4f2');
  });
});

describe('calculateInitials', () => {
  test('calculates initials correctly', () => {
    expect(calculateInitials('Ferris Rustacean')).toBe('FR');
    expect(calculateInitials('Ferris')).toBe('FE');
    expect(calculateInitials('F')).toBe('F');
    expect(calculateInitials('Ferris   Rustacean')).toBe('FR');
    expect(calculateInitials('Ferris rustacean')).toBe('FR');
    expect(calculateInitials('')).toBe('??');
  });
});

describe('formatDate function', () => {
  const isoDate = '2024-07-22T00:00:00Z';

  test('format with en-US locale', () => {
    expect(formatDate(isoDate, 'en-US')).toEqual('Jul 22, 2024');
  });

  test('format with en-GB locale', () => {
    expect(formatDate(isoDate, 'en-GB')).toEqual('22 Jul 2024');
  });

  test('format with de-DE locale', () => {
    expect(formatDate(isoDate, 'de-DE')).toEqual('22.07.2024');
  });

  test('format with nl-NL locale', () => {
    expect(formatDate(isoDate, 'nl-NL')).toEqual('22 jul 2024');
  });
});

import { calculateInitials, formatDate, formatDateTime, formatRelativeDateTime, hash } from './utils';

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
    expect(formatDate(isoDate, 'en-US', true)).toEqual('Jul 22, 2024');
  });

  test('format with en-GB locale', () => {
    expect(formatDate(isoDate, 'en-GB', true)).toEqual('22 Jul 2024');
  });

  test('format with de-DE locale', () => {
    expect(formatDate(isoDate, 'de-DE', true)).toEqual('22.07.2024');
  });

  test('format with nl-NL locale', () => {
    expect(formatDate(isoDate, 'nl-NL', true)).toEqual('22 jul 2024');
  });
});

describe('formatDateTime function', () => {
  const isoDate = '2024-07-22T09:31:45Z';

  test('format with en-US locale', () => {
    expect(formatDateTime(isoDate, 'en-US', true)).toEqual('Jul 22, 2024, 11:31:45 AM');
  });

  test('format with en-GB locale', () => {
    expect(formatDateTime(isoDate, 'en-GB', true)).toEqual('22 Jul 2024, 11:31:45');
  });

  test('format with de-DE locale', () => {
    expect(formatDateTime(isoDate, 'de-DE', true)).toEqual('22.07.2024, 11:31:45');
  });

  test('format with nl-NL locale', () => {
    expect(formatDateTime(isoDate, 'nl-NL', true)).toEqual('22 jul 2024, 11:31:45');
  });
});

describe('formatRelativeDateTime function', () => {
  test('1 min ago en-US', () => {
    const now = new Date();
    const oneMinuteAgo = new Date(now.setMinutes(now.getMinutes() - 1));
    expect(formatRelativeDateTime(oneMinuteAgo.toISOString(), 'en-US')).toEqual('1 minute ago');
  });

  test('2 mins ago en-US', () => {
    const now = new Date();
    const twoMinutesAgo = new Date(now.setMinutes(now.getMinutes() - 2));
    expect(formatRelativeDateTime(twoMinutesAgo.toISOString(), 'en-US')).toEqual('2 minutes ago');
  });

  test('1 hour ago en-GB', () => {
    const now = new Date();
    const oneHourAgo = new Date(now.setHours(now.getHours() - 1));
    expect(formatRelativeDateTime(oneHourAgo.toISOString(), 'en-GB')).toEqual('1 hour ago');
  });

  test('2 hours ago en-GB', () => {
    const now = new Date();
    const twoHoursAgo = new Date(now.setHours(now.getHours() - 2));
    expect(formatRelativeDateTime(twoHoursAgo.toISOString(), 'en-GB')).toEqual('2 hours ago');
  });

  test('1 day ago de-DE', () => {
    const now = new Date();
    const oneDayAgo = new Date(now.setDate(now.getDate() - 1));
    expect(formatRelativeDateTime(oneDayAgo.toISOString(), 'de-DE')).toEqual('Gestern');
  });

  test('2 days ago de-DE', () => {
    const now = new Date();
    const twoDaysAgo = new Date(now.setDate(now.getDate() - 2));
    expect(formatRelativeDateTime(twoDaysAgo.toISOString(), 'de-DE')).toEqual('Vorgestern');
  });

  test('3 days ago de-DE', () => {
    const now = new Date();
    const threeDaysAgo = new Date(now.setDate(now.getDate() - 3));
    expect(formatRelativeDateTime(threeDaysAgo.toISOString(), 'de-DE')).toEqual('Vor 3 Tagen');
  });

  test('1 month ago nl-NL', () => {
    const now = new Date();
    const oneMonthAgo = new Date(now.setMonth(now.getMonth() - 1));
    expect(formatRelativeDateTime(oneMonthAgo.toISOString(), 'nl-NL')).toEqual('Vorige maand');
  });

  test('2 months ago nl-NL', () => {
    const now = new Date();
    const twoMonthsAgo = new Date(now.setMonth(now.getMonth() - 2));
    expect(formatRelativeDateTime(twoMonthsAgo.toISOString(), 'nl-NL')).toEqual('2 maanden geleden');
  });

  test('1 year ago en-US', () => {
    const now = new Date();
    const oneYearAgo = new Date(now.setFullYear(now.getFullYear() - 1));
    expect(formatRelativeDateTime(oneYearAgo.toISOString(), 'en-US')).toEqual('Last year');
  });

  test('2 years ago en-US', () => {
    const now = new Date();
    const twoYearsAgo = new Date(now.setFullYear(now.getFullYear() - 2));
    expect(formatRelativeDateTime(twoYearsAgo.toISOString(), 'en-US')).toEqual('2 years ago');
  });
});

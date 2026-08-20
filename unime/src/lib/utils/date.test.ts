import { formatIso8601, parseIso8601 } from './date';

describe('parseIso8601', () => {
  test('parses a date without a time component', () => {
    expect(parseIso8601('1985-05-21')).toEqual({ date: '1985-05-21', hasTime: false });
  });

  test('parses a date with a time component', () => {
    expect(parseIso8601('2019-11-05T14:30:00Z')).toEqual({ date: '2019-11-05', hasTime: true });
    expect(parseIso8601('2019-11-05T14:30')).toEqual({ date: '2019-11-05', hasTime: true });
    expect(parseIso8601('2019-11-05T14:30:00.123+02:00')).toEqual({ date: '2019-11-05', hasTime: true });
  });

  test('treats midnight as a date without a time component', () => {
    expect(parseIso8601('2010-01-01T00:00:00Z')).toEqual({ date: '2010-01-01', hasTime: false });
    expect(parseIso8601('2010-01-01T00:00')).toEqual({ date: '2010-01-01', hasTime: false });
    expect(parseIso8601('2010-01-01T00:00:00.000+02:00')).toEqual({ date: '2010-01-01', hasTime: false });
  });

  test('rejects values that are not ISO 8601 timestamps', () => {
    expect(parseIso8601('Ferris')).toBeNull();
    expect(parseIso8601('21-05-1985')).toBeNull();
    expect(parseIso8601('1985')).toBeNull();
    // A date that does not exist.
    expect(parseIso8601('1985-02-30')).toBeNull();
    // Values that are not strings.
    expect(parseIso8601(1985)).toBeNull();
    expect(parseIso8601(undefined)).toBeNull();
    expect(parseIso8601({ birthdate: '1985-05-21' })).toBeNull();
  });
});

describe('formatIso8601', () => {
  test('formats a date without a time component in the given locale', () => {
    expect(formatIso8601('1985-05-21', 'en-US')).toEqual('May 21, 1985');
    expect(formatIso8601('1985-05-21', 'de-DE')).toEqual('21. Mai 1985');
    expect(formatIso8601('1985-05-21', 'nl-NL')).toEqual('21 mei 1985');
  });

  test('formats a date with a time component in the given locale', () => {
    // Timezone in CI is UTC, which would fail tests.
    expect(formatIso8601('2019-11-05T14:30:00Z', 'en-US', 'Europe/Amsterdam')).toEqual('November 5, 2019 at 3:30 PM');
    expect(formatIso8601('2019-11-05T14:30:00Z', 'de-DE', 'Europe/Amsterdam')).toEqual('5. November 2019 um 15:30');
  });

  test('formats a date without a time component independent of the timezone', () => {
    // Midnight UTC is the previous day in Los Angeles, but the date the issuer wrote down should be displayed.
    expect(formatIso8601('1985-05-21', 'en-US', 'America/Los_Angeles')).toEqual('May 21, 1985');
    expect(formatIso8601('1985-05-21T00:00:00Z', 'en-US', 'America/Los_Angeles')).toEqual('May 21, 1985');
  });

  test('returns values that are not ISO 8601 timestamps unchanged', () => {
    expect(formatIso8601('Ferris', 'en-US')).toEqual('Ferris');
  });
});

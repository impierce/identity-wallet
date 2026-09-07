import { findCountry } from './country';

describe('findCountry', () => {
  test('resolves the country of a field that holds a country code', () => {
    expect(findCountry('country', 'NL')).toEqual({ code: 'NL', name: 'Netherlands' });
    expect(findCountry('countryOfBirth', 'DE')).toEqual({ code: 'DE', name: 'Germany' });
    expect(findCountry('issuing_country', 'us')).toEqual({ code: 'US', name: 'United States of America' });
  });

  test('ignores fields that do not mention a country', () => {
    expect(findCountry('nationality', 'NL')).toBeNull();
    expect(findCountry('givenName', 'NL')).toBeNull();
  });

  test('ignores values that are not country codes', () => {
    expect(findCountry('country', 'Netherlands')).toBeNull();
    expect(findCountry('country', 'XX')).toBeNull();
    expect(findCountry('country', 528)).toBeNull();
    expect(findCountry('country', undefined)).toBeNull();
  });
});

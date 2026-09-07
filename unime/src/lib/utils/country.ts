import countries, { type Country } from '$lib/components/forms/countries';

/**
 * Field names that hold a country, e.g. `country`, `countryOfBirth` or `issuing_country`.
 *
 * The `country` field type of a credential template is not preserved when the credential is issued, so the field name
 * is the only hint left that a value is a country code.
 */
const COUNTRY_FIELD_REGEX = /country/i;

/**
 * Resolves the country a claim refers to, or `null` if the claim does not hold a country. Both the field name and the
 * value have to point at a country: the name has to mention a country and the value has to be an
 * [ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) code.
 */
export function findCountry(key: string, value: unknown): Country | null {
  if (!COUNTRY_FIELD_REGEX.test(key) || typeof value !== 'string') {
    return null;
  }

  const code = value.trim().toUpperCase();

  return countries.find((country) => country.code === code) ?? null;
}

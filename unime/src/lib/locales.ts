import type { Locale } from '@bindings/profile_settings/Locale';

export const locales: {
  locale: Locale;
  displayName: string;
}[] = [
  { locale: 'en-US', displayName: 'English (US)' },
  { locale: 'en-GB', displayName: 'English (UK)' },
  { locale: 'nl-NL', displayName: 'Nederlands (Nederland)' },
  { locale: 'de-DE', displayName: 'Deutsch (Deutschland)' },
  { locale: 'es-ES', displayName: 'Español (España)' },
  { locale: 'fi-FI', displayName: 'Suomi (Suomi)' },
  { locale: 'sv-FI', displayName: 'Svenska (Finland)' },
];

// Incomplete locales can be disabled here
export const disabledLocales: Locale[] = [];

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
];

// To disable a locale, it can be added to the array
export const incompleteLocales: Locale[] = ['en-GB'];
// TODO: extended locales (such as 'en-GB') cannot be enabled until this issue is resolved:
// https://github.com/ivanhofer/typesafe-i18n/issues/741

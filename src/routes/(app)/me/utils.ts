import type { Locale } from 'src-tauri/identity_wallet/bindings/Locale';

import { dispatch } from '$src/lib/dispatcher';

export const calculateInitials = (name: string): string => {
  let parts = name.split(' ').filter((n) => n.length > 0);
  if (parts.length === 1) {
    // Take first two letters, if only one name
    return parts.at(0)!!.slice(0, 2).toUpperCase();
  } else {
    let first = parts?.at(0)?.charAt(0) ?? '?';
    let last = parts?.at(1)?.charAt(0) ?? '?';
    // initials = names?.at(0)?.charAt(0) ?? '' + names?.at(1)?.charAt(0) ?? '';
    return `${first}${last}`.toUpperCase();
  }
};

export const languages: { locale: string; displayName: string }[] = [
  { locale: 'en', displayName: 'English (US)' },
  { locale: 'de', displayName: 'Deutsch' },
  { locale: 'nl', displayName: 'Nederlands' },
];

export const setNextLanguage = (current: Locale) => {
  const locales = languages.map((l) => l.locale);
  const next: string = locales[(locales.indexOf(current) + 1) % locales.length];
  dispatch({ type: '[Settings] Set locale', payload: { locale: next } });
};

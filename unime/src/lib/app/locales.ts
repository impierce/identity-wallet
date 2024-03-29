import type { SvelteComponent } from 'svelte';

import type { SvelteHTMLElements } from 'svelte/elements';

import type { Locale } from '@bindings/Locale';

import DE from '~icons/circle-flags/de';
import GB from '~icons/circle-flags/gb';
import NL from '~icons/circle-flags/nl';
import US from '~icons/circle-flags/us';

export const locales: {
  locale: Locale;
  flag: typeof SvelteComponent<SvelteHTMLElements['svg']>;
  displayName: string;
}[] = [
  { locale: 'en-US', flag: US, displayName: 'English (US)' },
  { locale: 'en-GB', flag: GB, displayName: 'English (UK)' },
  { locale: 'nl-NL', flag: NL, displayName: 'Nederlands' },
  { locale: 'de-DE', flag: DE, displayName: 'Deutsch' },
];

// To disable a locale, it can be added to the array
export const incompleteLocales: Locale[] = ['en-GB'];
// TODO: extended locales (such as 'en-GB') cannot be enabled until this issue is resolved:
// https://github.com/ivanhofer/typesafe-i18n/issues/741

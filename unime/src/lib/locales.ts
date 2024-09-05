import type { SvelteComponent } from 'svelte';

import type { SvelteHTMLElements } from 'svelte/elements';

import type { Locale } from '@bindings/profile_settings/Locale';

import { DEFlagIcon, GBFlagIcon, NLFlagIcon, USFlagIcon } from '$lib/icons';

export const locales: {
  locale: Locale;
  flag: typeof SvelteComponent<SvelteHTMLElements['svg']>;
  displayName: string;
}[] = [
  { locale: 'en-US', flag: USFlagIcon, displayName: 'English (US)' },
  { locale: 'en-GB', flag: GBFlagIcon, displayName: 'English (UK)' },
  { locale: 'nl-NL', flag: NLFlagIcon, displayName: 'Nederlands' },
  { locale: 'de-DE', flag: DEFlagIcon, displayName: 'Deutsch' },
];

// To disable a locale, it can be added to the array
export const incompleteLocales: Locale[] = ['en-GB'];
// TODO: extended locales (such as 'en-GB') cannot be enabled until this issue is resolved:
// https://github.com/ivanhofer/typesafe-i18n/issues/741

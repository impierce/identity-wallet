import type { SvelteComponent } from 'svelte';

import type { Locale } from 'src-tauri/identity_wallet/bindings/Locale';
import type { SvelteHTMLElements } from 'svelte/elements';

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

export const incompleteLocales: Locale[] = ['en-GB'];

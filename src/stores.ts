import { readable } from 'svelte/store';
import { listen } from '@tauri-apps/api/event';
import { navigate } from 'svelte-navigator';
import { setLocale } from './i18n/i18n-svelte';
import type { TransferState as State } from '../src-tauri/bindings/TransferState'; // TODO: copy to /src instead of importing across root

export const state = readable<State>(undefined, (set) => {
  listen('state-changed', (event) => {
    console.log(event);
    const state = event.payload as State;

    set(state);
    console.log(state);

    setLocale(state.locale);

    if (state.active_profile === null) {
      navigate('welcome');
    } else {
      navigate('profile');
    }
  });
  // TODO: unsubscribe from listener?
});

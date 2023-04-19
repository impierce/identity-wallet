import { readable } from 'svelte/store';
import { listen } from '@tauri-apps/api/event';
import { navigate } from 'svelte-navigator';
import { setLocale } from './i18n/i18n-svelte';
import type { Locales } from './i18n/i18n-types';
// TODO: run some copy task instead of importing across root to make the frontend independent
import type { TransferState as State } from 'src-tauri/bindings/TransferState';

interface StateChangedEvent {
  event: string;
  windowLabel: string;
  payload: State;
  id: number;
}

export const state = readable<State>(undefined, (set) => {
  listen('state-changed', (event: StateChangedEvent) => {
    const state = event.payload;

    set(state);
    console.log(state);

    setLocale(state.locale as Locales);

    if (state.active_profile === null) {
      navigate('welcome');
    } else {
      navigate('profile');
    }
  });
  // TODO: unsubscribe from listener necessary?
});

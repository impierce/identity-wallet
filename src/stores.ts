import { readable } from 'svelte/store';
import { listen } from '@tauri-apps/api/event';
import { goto } from '$app/navigation';
import { setLocale } from './i18n/i18n-svelte';
import type { Locales } from './i18n/i18n-types';
// TODO: run some copy task instead of importing across root to make the frontend independent
import type { TransferState as State } from '../src-tauri/bindings/TransferState';

interface StateChangedEvent {
  event: string;
  windowLabel: string;
  payload: State;
  id: number;
}

// const unlisten = await listen<string>('error', (event) => {
//   console.log(`Got error in window ${event.windowLabel}, payload: ${event.payload}`);
// });

export const state = readable<State>(undefined, (set) => {
  listen('state-changed', (event: StateChangedEvent) => {
    const state = event.payload;

    set(state);
    console.log(state);

    setLocale(state.locale as Locales);

    if (state.active_profile === null) {
      goto('welcome');
    } else {
      goto('profile');
    }
  });
  // TODO: unsubscribe from listener!
});

export const alert = readable<any>(undefined, (set) => {
  listen('interaction-required', (event: StateChangedEvent) => {
    console.log(event.payload);
    set(event.payload);
  });
});

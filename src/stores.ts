import { readable } from 'svelte/store';
import { listen } from '@tauri-apps/api/event';
import { goto } from '$app/navigation';
import { setLocale } from './i18n/i18n-svelte';
import type { Locales } from './i18n/i18n-types';
// TODO: run some copy task instead of importing across root to make the frontend independent
import type { TransferState as State } from '../src-tauri/bindings/TransferState';
import type { Redirect } from '../src-tauri/bindings/user-flow/Redirect';
import { debug, info } from '@tauri-apps/plugin-log';

interface StateChangedEvent {
  event: string;
  windowLabel: string;
  payload: State;
  id: number;
}

export const state = readable<State>(undefined, (set) => {
  const unlisten = listen('state-changed', (event: StateChangedEvent) => {
    const state = event.payload;

    set(state);
    info(`stores.ts: ${JSON.stringify(state)}`);

    setLocale(state.locale as Locales);

    if (state.current_user_flow?.type === 'redirect') {
      const redirect_target = (state.current_user_flow as Redirect).target;
      info(`redirecting to: "${redirect_target}"`);
      goto(redirect_target);
    }
  });
  // TODO: unsubscribe from listener!
});

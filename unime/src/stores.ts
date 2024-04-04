import { goto } from '$app/navigation';
import { writable } from 'svelte/store';

// TODO: run some copy task instead of importing across root to make the frontend independent
import type { AppState as State } from '@bindings/AppState';
import { listen } from '@tauri-apps/api/event';
import { info } from '@tauri-apps/plugin-log';

import { setLocale } from '$src/i18n/i18n-svelte';
import type { Locales } from '$src/i18n/i18n-types';

interface StateChangedEvent {
  event: string;
  windowLabel: string;
  payload: State;
  id: number;
}

// TODO: even needed? or simply "writable<State>(undefined, (set) => {});"?
const empty_state: State = {
  active_profile: {
    name: '',
    picture: null,
    theme: 'system',
    primary_did: '',
  },
  locale: 'en-US',
  credentials: [],
  current_user_prompt: null,
  dev_mode: 'Off',
  debug_messages: [],
  user_journey: null,
  connections: [],
  user_data_query: [],
};

/**
 * A read-only state that is updated by the Rust backend.
 * If the frontend intends to change the state, it must dispatch an action to the backend.
 */
// TODO: make read-only
export const state = writable<State>(undefined, (set) => {
  const unlisten = listen('state-changed', (event: StateChangedEvent) => {
    const state = event.payload;

    set(state);
    setLocale(state.profile_settings.locale as Locales);

    if (state.current_user_prompt?.type === 'redirect') {
      const redirect_target = state.current_user_prompt.target;
      info(`Redirecting to: "/${redirect_target}"`);
      goto(`/${redirect_target}`);
    }
  });
  // TODO: unsubscribe from listener!
});

/**
 * A state only used by the frontend for storing display logic.
 * Never touches the Rust backend and is therefore not persisted across app restarts.
 * This is useful during the onboarding process,
 * where the user sets their preferences and only in the last step they are pushed to the backend.
 */
export const onboarding_state = writable<OnboardingState>({});

interface OnboardingState {
  name?: string;
  password?: string; // TODO: secure enough?
}

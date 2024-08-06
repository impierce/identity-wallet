import { writable } from 'svelte/store';

// TODO: run some copy task instead of importing across root to make the frontend independent
import type { AppState } from '@bindings/AppState';
import { listen } from '@tauri-apps/api/event';
import { error as err } from '@tauri-apps/plugin-log';

interface ErrorEvent {
  event: string;
  payload: string;
  id: number;
}

interface OnboardingState {
  name?: string;
  password?: string; // TODO: security: is it okay to keep the password temporarily in the object?
}

const empty_state: AppState = {
  dids: {},
  connections: [],
  credentials: [],
  search_results: {
    current: [],
    recent_credentials: [],
  },
  profile_settings: {
    locale: 'en-US',
    profile: null,
    preferred_did_methods: ['did:jwk'],
    preferred_key_types: ['Ed25519'],
    sorting_preferences: {
      connections: {
        sort_method: 'name_az',
        reverse: false,
      },
      credentials: {
        sort_method: 'name_az',
        reverse: false,
      },
    },
  },
  current_user_prompt: null,
  user_journey: null,
  debug_messages: [],
  history: [],
  dev_mode: 'Off',
};

/**
 * A store that listens for updates to the application state emitted by the Rust backend.
 * If the frontend intends to change the state, it must dispatch an action to the backend.
 */
// TODO: make read-only
export const state = writable<AppState>(empty_state);

/**
 * A store that listens for errors emitted by the Rust backend.
 */
export const error = writable<string | undefined>(undefined, (set) => {
  listen('error', (event: ErrorEvent) => {
    err(`Error: ${event.payload}`);
    set(event.payload);
  });
  // TODO: unsubscribe from listener!
});

/**
 * This store is only used by the frontend for storing state during onboarding.
 * The data never touches the Rust backend and is therefore not persisted across app restarts.
 * The state of the onboarding is pushed to the backend only on the last of the onboarding process.
 */
export const onboarding_state = writable<OnboardingState>({});

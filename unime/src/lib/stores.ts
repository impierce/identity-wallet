import { writable } from 'svelte/store';

// TODO: run some copy task instead of importing across root to make the frontend independent
import type { AppState } from '@bindings/AppState';

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
 * This store contains the frontend state.
 * It may be altered only by the `state-changed` Tauri listener.
 * The frontend must dispatch an action to the backend to change state.
 */
// TODO: make read-only
export const state = writable<AppState>(empty_state);

/**
 * This store contains errors to be displayed by an error toast.
 * It may be altered only by the `error` Tauri listener.
 */
export const error = writable<string | undefined>(undefined);

/**
 * This store is only used by the frontend for storing state during onboarding.
 * The data never touches the Rust backend and is therefore not persisted across app restarts.
 * The state of the onboarding is pushed to the backend only on the last of the onboarding process.
 */
export const onboarding_state = writable<OnboardingState>({});

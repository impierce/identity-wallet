import type { AppState } from 'src-tauri/bindings/AppState';

export const testState: AppState = {
  active_profile: {
    name: 'Wright',
    picture: '&#x1F527',
    theme: 'dark',
    primary_did: '',
  },
  locale: 'de',
  credentials: [],
  current_user_prompt: null,
  dev_mode_enabled: false,
  debug_messages: [],
  user_journey: null,
  connections: [],
  user_data_query: [],
};

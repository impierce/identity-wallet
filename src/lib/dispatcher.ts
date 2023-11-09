import { invoke } from '@tauri-apps/api/primitives';
import { info } from '@tauri-apps/plugin-log';

import type { Action } from '../../src-tauri/bindings/Action';

/**
 * Dispatches an action to the Tauri backend.
 *
 * @param {Action} action
 */
export const dispatch = async (action: Action) => {
  info(`Dispatching action: ${JSON.stringify(action)}`);
  await invoke('handle_action', { action }).catch((err) => {
    console.error(err);
  });
};

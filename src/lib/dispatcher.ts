import { invoke } from '@tauri-apps/api/tauri';
import type { Action } from '../../src-tauri/bindings/Action';
import { info } from '@tauri-apps/plugin-log';

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

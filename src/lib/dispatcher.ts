import { invoke } from '@tauri-apps/api/tauri';
import type { Action } from '../../src-tauri/bindings/Action';

/**
 * Dispatches an action to the Tauri backend.
 *
 * @param {Action} action
 */
export const dispatch = async (action: Action) => {
  console.log('Dispatching action: %j', action);
  await invoke('handle_action', { action }).catch((err) => {
    console.error(err);
  });
};

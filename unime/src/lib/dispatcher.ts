import type { Action } from '@bindings/actions/Action';
import { invoke } from '@tauri-apps/api/core';
import { error, info } from '@tauri-apps/plugin-log';

/**
 * Dispatches an action to the Tauri backend.
 *
 * @param {Action} action
 */
export const dispatch = async (action: Action) => {
  // Only the type: payloads can contain credentials, codes and personal data.
  info(`Dispatching action: ${action.type}`);
  await invoke('handle_action', { action }).catch((err) => {
    error(err);
  });
};

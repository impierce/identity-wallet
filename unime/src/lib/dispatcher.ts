import type { Action } from '@bindings/actions/Action';
import { invoke } from '@tauri-apps/api/core';
import { error, info } from '@tauri-apps/plugin-log';

import { sanitizeStringify } from './sensitive-logging';

/**
 * Dispatches an action to the Tauri backend.
 *
 * @param {Action} action
 */
export const dispatch = async (action: Action) => {
  info(`Dispatching action: ${sanitizeStringify(action)}`);
  await invoke('handle_action', { action }).catch((err) => {
    error(err);
  });
};

/**
 * Like {@link dispatch}, but rejects instead of swallowing the backend error.
 *
 * Use it where the outcome changes what the user sees — an incorrect backup
 * password, for example, is not something to log and carry on from.
 */
export const tryDispatch = async (action: Action) => {
  info(`Dispatching action: ${sanitizeStringify(action)}`);
  try {
    await invoke('handle_action', { action });
  } catch (err) {
    error(String(err));
    throw err;
  }
};

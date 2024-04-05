import type { HandleClientError } from '@sveltejs/kit';
import { error, info } from '@tauri-apps/plugin-log';

export const handleError = (async ({ error: err, event }) => {
  info(`hooks.client.ts: event.url: "${event.url}"`);

  const errorId = crypto.randomUUID();

  error(`${errorId} - ${err.message}`);

  return {
    message: err.message,
    errorId,
  };
}) satisfies HandleClientError;

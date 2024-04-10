import type { HandleClientError } from '@sveltejs/kit';
import { info } from '@tauri-apps/plugin-log';

export const handleError = (async ({ error, event }) => {
  info(`hooks.client.ts: event.url: "${event.url}"`);

  const errorId = crypto.randomUUID();

  // eslint-disable-next-line no-console
  console.error(error);

  return {
    message: error.message,
    errorId,
  };
}) satisfies HandleClientError;

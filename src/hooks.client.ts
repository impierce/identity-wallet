import type { HandleClientError } from '@sveltejs/kit';
import { info } from '@tauri-apps/plugin-log';

export const handleError = (async ({ error, event }) => {
  info(`hooks.client.ts: event.url: "${event.url}"`);

  const errorId = crypto.randomUUID();
  // example integration with https://sentry.io/
  // Sentry.captureException(error, { extra: { event, errorId } });

  console.error(error);

  return {
    message: 'Whoops!',
    errorId
  };
}) satisfies HandleClientError;

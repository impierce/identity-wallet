import type { HandleClientError } from '@sveltejs/kit';

export const handleError = (async ({ error, event }) => {
  console.log(`hooks.client.ts: event.url: "${event.url}"`);

  const errorId = crypto.randomUUID();
  // example integration with https://sentry.io/
  // Sentry.captureException(error, { extra: { event, errorId } });

  return {
    message: 'Whoops!',
    errorId
  };
}) satisfies HandleClientError;

import type { Handle, HandleServerError } from '@sveltejs/kit';

export const handle = (async ({ event, resolve }) => {
  // info(`hooks.server.ts: event.url: "${event.url}"`);
  console.log(`hooks.server.ts: event.url: "${event.url}"`);

  const response = await resolve(event);

  // info(`hooks.server.ts: response.url: "${response.url}"`);
  return response;
}) satisfies Handle;

export const handleError: HandleServerError = async ({ error, event, status, message }) => {
  const errorId = crypto.randomUUID();

  // example integration with https://sentry.io/
  // Sentry.captureException(error, {
  //   extra: { event, errorId, status },
  // });

  console.error(error);

  return {
    message: 'Whoops!',
    errorId,
  };
};

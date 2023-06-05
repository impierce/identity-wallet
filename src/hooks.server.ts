import type { Handle } from '@sveltejs/kit';

export const handle = (async ({ event, resolve }) => {
  console.log(`hooks.server.ts: event.url: "${event.url}"`);

  const response = await resolve(event);

  console.log(`hooks.server.ts: response.url: "${response.url}"`);
  return response;
}) satisfies Handle;

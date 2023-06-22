import type { Handle } from '@sveltejs/kit';
// import { info } from '@tauri-apps/plugin-log';

export const handle = (async ({ event, resolve }) => {
  // info(`hooks.server.ts: event.url: "${event.url}"`);
  console.log(`hooks.server.ts: event.url: "${event.url}"`);

  const response = await resolve(event);

  // info(`hooks.server.ts: response.url: "${response.url}"`);
  return response;
}) satisfies Handle;

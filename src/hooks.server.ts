import type { Handle } from '@sveltejs/kit';

export const handle = (async ({ event, resolve }) => {
  // if (event.url.pathname.startsWith('/custom')) {
  //     return new Response('custom response');
  // }

  //   console.log('hooks.server.ts: event:', event);

  const response = await resolve(event);

  console.log('hooks.server.ts: response:', response.url);
  return response;
}) satisfies Handle;

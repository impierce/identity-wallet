import type { Handle, HandleServerError } from '@sveltejs/kit';

export const handle = (async ({ event, resolve }) => {
  const response = await resolve(event);
  return response;
}) satisfies Handle;

export const handleError: HandleServerError = async ({ error }) => {
  const errorId = crypto.randomUUID();

  // eslint-disable-next-line no-console
  console.error(error);

  return {
    message: error.message,
    errorId,
  };
};

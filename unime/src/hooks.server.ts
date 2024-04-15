import { dev } from '$app/environment';

import type { HandleServerError } from '@sveltejs/kit';
import { info } from '@tauri-apps/plugin-log';

// TODO: Refactor after upgrade to SvelteKit v2.
// `error.message` may contain sensitive information that should not be exposed.
// In v2 you get a safe `message` prop: https://kit.svelte.dev/docs/hooks#shared-hooks-handleerror.

export const handleError: HandleServerError = async ({ error, event }) => {
  info(`hooks.server.ts (handleError): event.url: "${event.url}"`);

  const errorId = crypto.randomUUID();

  if (dev) {
    // eslint-disable-next-line no-console
    console.error(error);
  }

  return {
    // Use type assertion to access `message` property on unknown type.
    message: (error as Error).message,
    errorId,
  };
};

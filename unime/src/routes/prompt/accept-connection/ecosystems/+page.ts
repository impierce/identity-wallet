import type { PageLoad } from './$types';

// No bottom button bar here, unlike the prompt page.
export const load = (async () => {
  return { bgAltBottom: false };
}) satisfies PageLoad;

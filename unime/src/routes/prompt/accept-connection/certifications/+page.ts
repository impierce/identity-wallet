import type { PageLoad } from './$types';

// The list page has no bottom button bar, unlike the prompt page it comes from.
export const load = (async () => {
  return { bgAltBottom: false };
}) satisfies PageLoad;

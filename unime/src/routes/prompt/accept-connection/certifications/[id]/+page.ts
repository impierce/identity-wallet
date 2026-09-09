import type { PageLoad } from './$types';

// Dynamic IDs are not known at build time.
export const prerender = false;

// The detail page has no bottom button bar, unlike the prompt page it comes from.
export const load = (async () => {
  return { bgAltBottom: false };
}) satisfies PageLoad;

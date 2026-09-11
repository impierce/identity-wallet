import type { PageLoad } from './$types';

// Dynamic IDs are not known at build time.
export const prerender = false;

// No bottom button bar here, unlike the prompt page.
export const load = (async () => {
  return { bgAltBottom: false };
}) satisfies PageLoad;

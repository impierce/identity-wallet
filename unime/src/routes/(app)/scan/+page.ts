import type { PageLoad } from './$types';

export const load = (async () => {
  // Ask root layout to be transparent.
  return { transparent: true, bgAltTop: true, bgAltBottom: true };
}) satisfies PageLoad;

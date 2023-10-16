import type { LayoutLoad } from './$types';

export const load: LayoutLoad = ({ url }) => {
  const { pathname } = url;
  console.log('+layout.ts: pathname', pathname);
  return {
    pathname,
  };
};

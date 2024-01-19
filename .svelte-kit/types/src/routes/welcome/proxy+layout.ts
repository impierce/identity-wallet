// @ts-nocheck
import type { LayoutLoad } from './$types';

export const load = ({ url }: Parameters<LayoutLoad>[0]) => {
  const { pathname } = url;
  console.log('+layout.ts: pathname', pathname);
  return {
    pathname,
  };
};

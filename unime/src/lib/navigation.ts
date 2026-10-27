import { beforeNavigate, goto } from '$app/navigation';
import { page } from '$app/state';

import { dispatch } from '$lib/dispatcher';

const RETURN_PARAM = 'from';

/**
 * Hierarchical back navigation: "back" always leads to the page's parent route, never to whatever happens to be the
 * previous history entry. This way back never re-enters a step that was already completed (e.g. a form that was
 * submitted) and it behaves the same for the back button in the top bar and the Android system back gesture.
 *
 * When the previous history entry is the parent route, back simply returns to it, so the parent keeps its state
 * (e.g. the selected tab) and no duplicate history entries pile up. Otherwise the current entry is replaced by the
 * parent route.
 *
 * Must be called during component initialization. Returns a function to navigate to the parent route, to be used
 * by the back button in the top bar and after completing an action on the page.
 *
 * @param parentRoute The route to go back to, or a function returning it (for routes that depend on page params).
 */
export function useParentRoute(parentRoute: string | (() => string)): () => void {
  const parentUrl = () => new URL(typeof parentRoute === 'function' ? parentRoute() : parentRoute, location.href);

  interceptBack(
    () => goto(parentUrl(), { replaceState: true }),
    (to) => {
      const parent = parentUrl();
      return to?.pathname === parent.pathname && to.search === parent.search;
    },
  );

  return () => history.back();
}

/**
 * Like `useParentRoute`, but runs `onBack` instead of navigating to a parent route. Used by user prompts, where going
 * back means cancelling the flow (the backend then redirects).
 *
 * Must be called during component initialization. Returns `onBack`, to be used by the back button in the top bar.
 */
export function useBackNavigation<T>(onBack: () => T): () => T {
  interceptBack(onBack, () => false);

  return onBack;
}

function interceptBack(onBack: () => unknown, isParent: (to: URL | undefined) => boolean) {
  beforeNavigate(({ type, delta, to, cancel }) => {
    // Only backward traversal is "back": forward navigation (`delta > 0`) goes to its history entry as usual.
    if (type !== 'popstate' || !delta || delta > 0 || isParent(to?.url)) return;
    // On popstate the browser already points at the previous entry. Cancelling makes SvelteKit restore the current
    // entry with `history.go()`, which is asynchronous: navigating before it lands would race with it (losing the
    // first back press), so we wait for the restoring popstate first.
    cancel();
    addEventListener('popstate', () => onBack(), { once: true });
  });
}

/**
 * Link to `href` whose back navigation returns to the current page instead of `href`'s own parent route. Used for
 * cross-links, such as from a credential to its issuer's connection page.
 */
export function linkWithReturn(href: string): string {
  const url = new URL(href, location.href);
  url.searchParams.set(RETURN_PARAM, page.url.pathname + page.url.search);
  return url.pathname + url.search;
}

/**
 * The page that linked to the current page with `linkWithReturn`, or `fallback` if there is none.
 */
export function returnRoute(fallback: string): string {
  const from = page.url.searchParams.get(RETURN_PARAM);
  if (!from?.startsWith('/') || from.startsWith('//')) return fallback;
  // Browsers also read e.g. `/\host` as a link to another host.
  const url = new URL(from, location.href);
  return url.origin === location.origin ? url.pathname + url.search : fallback;
}

/**
 * Cancels the active user flow (e.g. a connection request or credential offer) and leaves it for the "Me" page,
 * replacing the prompt's history entry so that back cannot return into the cancelled flow.
 */
export async function cancelUserFlow() {
  await dispatch({ type: '[User Flow] Cancel', payload: { redirect: 'me' } });
  await goto('/me', { replaceState: true });
}

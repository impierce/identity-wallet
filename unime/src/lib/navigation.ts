import { beforeNavigate, goto } from '$app/navigation';

import { dispatch } from '$lib/dispatcher';

/**
 * Hierarchical back navigation: "back" always leads to the page's parent route, never to whatever happens to be the
 * previous history entry. This way back never re-enters a step that was already completed (e.g. a form that was
 * submitted) and it behaves the same for the back button in the top bar and the Android system back gesture.
 *
 * Must be called during component initialization. Returns a function to navigate to the parent route, to be used
 * by the back button in the top bar and after completing an action on the page.
 *
 * @param parentRoute The route to go back to, or a function returning it (for routes that depend on page params).
 */
export function useParentRoute(parentRoute: string | (() => string)): () => Promise<void> {
  return useBackNavigation(() =>
    goto(typeof parentRoute === 'function' ? parentRoute() : parentRoute, { replaceState: true }),
  );
}

/**
 * Like `useParentRoute`, but runs `onBack` instead of navigating to a parent route. Used by user prompts, where going
 * back means cancelling the flow (the backend then redirects).
 *
 * Must be called during component initialization. Returns `onBack`, to be used by the back button in the top bar.
 */
export function useBackNavigation<T>(onBack: () => T): () => T {
  beforeNavigate(({ type, cancel }) => {
    if (type !== 'popstate') return;
    // On popstate the browser already points at the previous entry. Cancelling makes SvelteKit restore the current
    // entry with `history.go()`, which is asynchronous: navigating before it lands would race with it (losing the
    // first back press), so we wait for the restoring popstate first.
    cancel();
    addEventListener('popstate', () => onBack(), { once: true });
  });

  return onBack;
}

/**
 * Cancels the active user flow (e.g. a connection request or credential offer) and leaves it for the "Me" page,
 * replacing the prompt's history entry so that back cannot return into the cancelled flow.
 */
export async function cancelUserFlow() {
  await dispatch({ type: '[User Flow] Cancel', payload: { redirect: 'me' } });
  await goto('/me', { replaceState: true });
}

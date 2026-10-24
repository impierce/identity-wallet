import { beforeNavigate, goto } from '$app/navigation';

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
  const navigateUp = () =>
    goto(typeof parentRoute === 'function' ? parentRoute() : parentRoute, { replaceState: true });

  beforeNavigate(({ type, cancel }) => {
    if (type !== 'popstate') return;
    // On popstate the browser already points at the previous entry. Cancelling makes SvelteKit restore the current
    // entry with `history.go()`, which is asynchronous: navigating before it lands would race with it (losing the
    // first back press), so we wait for the restoring popstate first.
    cancel();
    addEventListener('popstate', navigateUp, { once: true });
  });

  return navigateUp;
}

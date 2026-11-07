import { onNavigate } from '$app/navigation';

import { platform } from '@tauri-apps/plugin-os';

export type PageTransitionDirection = 'forward' | 'back';

/**
 * Parent and child pages whose navigation slides, as a first set of examples. Every other navigation keeps switching
 * pages without a transition. Route IDs, as in `page.route.id`.
 */
const STACKS: [parent: string, child: string][] = [
  ['/(app)/me', '/credentials/[id]'],
  ['/(app)/me/search', '/credentials/[id]'],
  ['/(app)/activity', '/(app)/activity/connection/[id]'],
];

/** `forward` when going from a parent to its child page, `back` the other way around, otherwise `undefined`. */
export function pageTransitionDirection(
  from: string | null | undefined,
  to: string | null | undefined,
): PageTransitionDirection | undefined {
  for (const [parent, child] of STACKS) {
    if (from === parent && to === child) return 'forward';
    if (from === child && to === parent) return 'back';
  }
  return undefined;
}

/** iOS slides pages in from the side; everywhere else they follow Material's shared axis pattern. */
function transitionStyle(): 'ios' | 'material' {
  try {
    return platform() === 'ios' ? 'ios' : 'material';
  } catch {
    // Not running in Tauri, e.g. the frontend-only development server.
    return 'material';
  }
}

/**
 * Animates navigation between the pages in `STACKS` with the View Transitions API. The animations themselves are
 * defined in `app.css`, keyed on the `data-page-transition` and `data-page-transition-style` attributes set here.
 *
 * Must be called during component initialization, once, from the root layout.
 *
 * @param backdrop The element whose background shows between the pages while they cross-fade.
 */
export function usePageTransitions(backdrop: () => HTMLElement | undefined) {
  const root = document.documentElement;
  root.dataset.pageTransitionStyle = transitionStyle();

  onNavigate((navigation) => {
    const direction = pageTransitionDirection(navigation.from?.route.id, navigation.to?.route.id);
    if (!direction || !document.startViewTransition || window.matchMedia('(prefers-reduced-motion: reduce)').matches) {
      return;
    }

    root.dataset.pageTransition = direction;
    // The pages are captured as images, and the document underneath is not painted meanwhile: without a backdrop in
    // the theme's background colour, a cross-fade would pass through white in dark mode.
    const element = backdrop();
    if (element) root.style.setProperty('--page-transition-backdrop', getComputedStyle(element).backgroundColor);

    return new Promise<void>((resolve) => {
      const transition = document.startViewTransition(async () => {
        resolve();
        await navigation.complete;
      });
      transition.finished.finally(() => {
        delete root.dataset.pageTransition;
      });
    });
  });
}

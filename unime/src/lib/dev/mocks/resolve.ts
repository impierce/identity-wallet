import type { AppState } from '@bindings/AppState';
import type { CurrentUserPrompt } from '@bindings/user_prompt/CurrentUserPrompt';

import { mocks } from './accept-connection';

export type AcceptConnectionPrompt = Extract<CurrentUserPrompt, { type: 'accept-connection' }>;

/**
 * Returns the mock prompt named by `?mock=` when dev mode is on.
 *
 * Returns `null` when there is no active prompt, which happens after the user
 * accepts or cancels and the backend clears it.
 */
export function resolveAcceptConnectionPrompt(url: URL, appState: AppState): AcceptConnectionPrompt | null {
  // `import.meta.env.DEV` is replaced with `false` at build time, making this branch
  // unreachable in production. Note the fixtures are still present in the bundle:
  // Rollup does not tree-shake them out, verified against `vite build` output.
  if (import.meta.env.DEV) {
    const name = url.searchParams.get('mock');
    if (appState.dev_mode !== 'Off' && name && name in mocks) {
      return mocks[name as keyof typeof mocks];
    }
  }
  const prompt = appState.current_user_prompt;
  return prompt?.type === 'accept-connection' ? prompt : null;
}

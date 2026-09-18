import { PUBLIC_USE_MOCKS } from '$env/static/public';

import type { AppState } from '@bindings/AppState';
import type { CurrentUserPrompt } from '@bindings/user_prompt/CurrentUserPrompt';

import { mocks, type MockName } from './accept-connection';

export type AcceptConnectionPrompt = Extract<CurrentUserPrompt, { type: 'accept-connection' }>;

/** Off unless `PUBLIC_USE_MOCKS=true` is set in `.env`, so `?mock=` is inert by default. */
const useMocks = PUBLIC_USE_MOCKS === 'true';

/**
 * Returns the fixture named by `?mock=`, or `null` when the page is showing a real prompt.
 */
function selectMock(url: URL): AcceptConnectionPrompt | null {
  if (!useMocks) return null;
  const name = url.searchParams.get('mock');
  return name && name in mocks ? mocks[name as MockName] : null;
}

/**
 * True when the page is rendering a fixture rather than a real prompt.
 *
 * Gates the backend dispatches: a mocked page has no prompt for the backend to act on,
 * so accepting or cancelling one must stay client-side.
 */
export function isMockPrompt(url: URL): boolean {
  return selectMock(url) !== null;
}

/**
 * Returns the fixture named by `?mock=` when fixtures are switched on.
 *
 * Returns `null` when there is no active prompt, which happens after the user
 * accepts or cancels and the backend clears it.
 */
export function resolveAcceptConnectionPrompt(url: URL, appState: AppState): AcceptConnectionPrompt | null {
  const mock = selectMock(url);
  if (mock) return mock;
  const prompt = appState.current_user_prompt;
  return prompt?.type === 'accept-connection' ? prompt : null;
}

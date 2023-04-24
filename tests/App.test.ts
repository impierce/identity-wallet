import '@testing-library/jest-dom';

import { clearMocks, mockIPC } from '@tauri-apps/api/mocks';
import { vi } from 'vitest';
import App from '../src/routes/+page.svelte';
import { tick } from 'svelte';
import { render } from '@testing-library/svelte';

beforeEach(() => {
  // init __TAURI_IPC__
  mockIPC((cmd, args) => {});
});

afterEach(() => {
  clearMocks();
});

test('fetches app state on mount', async () => {
  const spy = vi.spyOn(window, '__TAURI_IPC__');

  await tick();

  render(App, {});

  expect(spy).toHaveBeenCalledTimes(1);
  expect(spy).toHaveBeenCalledWith({
    cmd: 'tauri',
    callback: expect.anything(),
    error: expect.anything(),
    message: {
      cmd: 'listen',
      event: 'state-changed',
      handler: expect.anything(),
      windowLabel: null
    },
    __tauriModule: expect.anything()
  });

  // TODO: assert action "[App] Get state" has been dispatched on mount
  // expect(spy).toHaveBeenCalledWith({
  //   action: { type: '[App] Get state' },
  //   callback: expect.anything(),
  //   cmd: 'execute_command',
  //   error: expect.anything()
  // });
});
